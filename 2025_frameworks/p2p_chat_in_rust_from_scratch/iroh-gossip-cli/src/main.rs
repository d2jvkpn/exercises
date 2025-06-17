use std::{fmt::Debug, path::Path, str::FromStr};

use iroh_gossip_cli::handlers::{input_loop, subscribe_loop};
use iroh_gossip_cli::structs::{Message, MessageBody, Ticket};
use iroh_gossip_cli::utils::iroh_secret_key;

use anyhow::{Result, anyhow};
use clap::{ArgAction, Args, Parser};
use iroh::{Endpoint, NodeAddr, RelayMap, RelayMode, RelayNode, RelayUrl, protocol::Router};
use iroh_gossip::{ALPN, net::Gossip, proto::TopicId};
use rand::prelude::*;

use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

/// Chat over iroh-gossip
///
/// This broadcasts unsigned messages over iroh-gossip.
///
/// By default a new node id is created when starting the example.
///
/// By default, we use the default n0 discovery services to dial by `NodeId`.
#[derive(Parser, Debug)]
#[command(name = "iroh-gossip-cli", version = "1.0", about = "p2p chat inrust from scratch")]
struct Command {
    #[clap(subcommand)]
    subcommand: Subcommand,

    #[clap(short, long, default_value = "configs/local.yaml")]
    config: String,

    #[clap(short, long)]
    relay_url: Option<String>,

    /*
    /// Set the bind port for our socket. By default, a random port will be used.
    #[clap(short, long, default_value = "0")]
    bind_port: u16,
    */
    /// Set your nickname.
    #[clap(short, long)]
    name: String,
}

#[derive(Parser, Debug)]
enum Subcommand {
    /// Open a chat room for a topic and print a ticket for others to join.
    Open,
    /// Join a chat room from a ticket.
    Join {
        /// The ticket, as base64 string.
        ticket: String,
    },
    // Join(JoinCommand),
}

#[derive(Debug, Args)]
struct JoinCommand {
    ticket: String,

    // --ticket t1 --ticket t2 --ticket t3
    #[arg(short = 't', long = "ticket", action = ArgAction::Append)]
    tickets_v1: Vec<String>,

    /// t1 t2 t3
    #[arg(required = true, num_args = 1..)]
    tickets_v2: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Command::parse();
    let name = args.name.clone();

    let (topic, ticket_nodes) = match &args.subcommand {
        Subcommand::Open => {
            let topic = TopicId::from_bytes(rand::random());
            println!("==> Opening chat room for topic {topic}");
            (topic, vec![])
        }
        Subcommand::Join { ticket } => {
            let Ticket { topic, nodes } = Ticket::from_str(&ticket)?;
            println!("==> Joining chat room for topic {topic}");
            (topic, nodes)
        }
    };

    let relay_map: RelayMap = args
        .relay_url
        .and_then(|v| Some(v.parse::<RelayUrl>().ok()?))
        .map(RelayNode::from)
        .map(RelayMap::from)
        .unwrap_or_else(|| RelayMap::empty());

    let endpoint = if relay_map.is_empty() {
        Endpoint::builder()
    } else {
        Endpoint::builder().relay_mode(RelayMode::Custom(relay_map))
    }
    .secret_key(iroh_secret_key())
    .discovery_n0()
    .bind()
    .await?;

    //let relay_url = endpoint.home_relay().initialized().await.unwrap();
    //println!("==> relay_url: {:?}", relay_url);

    let node_id = endpoint.node_id();
    // Get our address information, includes our `NodeId`, our `RelayUrl`, and any direct addresses.
    let node_addr = endpoint.node_addr().await?;

    // Build and instance of the gossip protocol and add a clone of the endpoint we have built.
    // The gossip protocol will use the endpoint to make connections.
    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;

    // The Router is how we manage protocols on top of the iroh endpoint. It handles all incoming
    // messages and routes them to the correct protocol.
    let router = Router::builder(endpoint.clone()).accept(ALPN, gossip.clone()).spawn();

    // in our main file, after we create a topic `id`:
    // print a ticket that includes our own node id and endpoint addresses

    let mut all_nodes: Vec<NodeAddr> =
        ticket_nodes.choose_multiple(&mut rand::rng(), 2).map(|x| (*x).clone()).collect();

    all_nodes.push(node_addr);

    let ticket = Ticket { topic, nodes: all_nodes };
    write_ticket(&ticket, &name).await?;

    // join the gossip topic by connecting to known nodes, if any
    let node_ids = ticket_nodes.iter().map(|p| p.node_id).collect();

    if ticket_nodes.is_empty() {
        println!("--> waiting for nodes to join us...");
    } else {
        // add the peer addrs from the ticket to our endpoint's addressbook,
        // so that they can be dialed
        for node in ticket_nodes.into_iter() {
            // println!("--> trying to connect to node: {:?}...", node);
            if let Err(e) = endpoint.add_node_addr(node.clone()) {
                println!("!!! can't connect to node: {e:?}");
            } else {
                println!("--> connected to node: {}", node.node_id);
            }
        }
    }

    let (sender, receiver) = gossip.subscribe_and_join(topic, node_ids).await?.split();
    println!("--> node(s) connected!");

    let message = Message::new(MessageBody::AboutMe { from: node_id, name: name.clone() });
    sender.broadcast(message.to_vec().into()).await?;

    tokio::spawn(subscribe_loop(node_id, name.clone(), sender.clone(), receiver));

    // spawn an input thread that reads stdin create a multi-provider, single-consumer channel
    let (line_tx, mut line_rx) = tokio::sync::mpsc::channel(1);
    // and pass the `sender` portion to the `input_loop`
    std::thread::spawn(move || input_loop(line_tx));

    // broadcast each line we type
    println!("==> type a message and hit enter to broadcast...");
    // listen for lines that we have typed to be sent from `stdin`
    while let Some(text) = line_rx.recv().await {
        // create a message from the text
        let message =
            Message::new(MessageBody::Message { from: endpoint.node_id(), text: text.clone() });
        // broadcast the encoded message
        sender.broadcast(message.to_vec().into()).await?;
        // print to ourselves the text that we sent
        println!(">>> You: {}", text.trim());
    }

    router.shutdown().await?;
    Ok(())
}

async fn write_ticket(ticket: &Ticket, name: &str) -> Result<()> {
    let node_addr = ticket.nodes.last().ok_or_else(|| anyhow!("nodes is empty"))?;

    let configs = Path::new("configs");
    fs::create_dir_all(configs).await?;

    let filepath = configs.join(format!("{}.ticket", name));
    let mut file = File::create(&filepath).await?;
    //file.write_all(&ticket.to_bytes()).await?;
    file.write_all(&ticket.to_bytes()).await?;
    file.write_all(b"\n").await?;
    // println!("--> node: {node_addr:?}\n    ticket: {ticket}");
    println!("--> node_id: {}", node_addr.node_id);
    println!("    filepath: {}", filepath.display());
    println!("    relay_url: {:?}", node_addr.relay_url());
    println!("    direct_addresses: {:?}", node_addr.direct_addresses().collect::<Vec<_>>());
    println!("    ticket: {ticket}");

    Ok(())
}
