use std::{collections::HashMap, fmt, str::FromStr};

use anyhow::Result;
use clap::{Args, Parser};
use data_encoding::BASE32_NOPAD;
use futures_lite::StreamExt;
use iroh::{Endpoint, NodeAddr, NodeId, PublicKey, SecretKey, protocol::Router};
use iroh_gossip::net::{Event, Gossip, GossipEvent, GossipReceiver, GossipSender};
use iroh_gossip::{ALPN, proto::TopicId};
use serde::{Deserialize, Serialize};

/// Chat over iroh-gossip
///
/// This broadcasts unsigned messages over iroh-gossip.
///
/// By default a new node id is created when starting the example.
///
/// By default, we use the default n0 discovery services to dial by `NodeId`.
#[derive(Parser, Debug)]
#[command(name = "myapp", version = "1.0", about = "A CLI application with subcommands")]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    // Set the bind port for our socket. By default, a random port will be used.
    //#[clap(short, long, default_value = "0")]
    //bind_port: u16,
    /// Set your nickname.
    #[clap(short, long)]
    name: String,
}

#[derive(Parser, Debug)]
enum Command {
    /// Open a chat room for a topic and print a ticket for others to join.
    Open,
    /// Join a chat room from a ticket.
    Join {
        /// The ticket, as base32 string.
        ticket: String,
    },
    // Join(JoinCommand),
}

#[derive(Debug, Args)]
struct JoinCommand {
    ticket: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let (topic, nodes) = match &args.command {
        Command::Open => {
            let topic = TopicId::from_bytes(rand::random());
            println!("==> Opening chat room for topic {topic}");
            (topic, vec![])
        }
        Command::Join { ticket } => {
            let Ticket { topic, nodes } = Ticket::from_str(&ticket)?;
            println!("==> Joining chat room for topic {topic}");
            (topic, nodes)
        }
    };

    let secret_key = SecretKey::generate(rand::rngs::OsRng);
    println!("--> secret_key: {secret_key}");

    let endpoint = Endpoint::builder().secret_key(secret_key).discovery_n0().bind().await?;
    println!("--> node_id: {:?}", endpoint.node_id());

    // Build and instance of the gossip protocol and add a clone of the endpoint we have built.
    // The gossip protocol will use the endpoint to make connections.
    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;

    // The Router is how we manage protocols on top of the iroh endpoint. It handles all incoming
    // messages and routes them to the correct protocol.
    let router = Router::builder(endpoint.clone()).accept(ALPN, gossip.clone()).spawn();

    // in our main file, after we create a topic `id`:
    // print a ticket that includes our own node id and endpoint addresses
    let ticket = {
        // Get our address information, includes our `NodeId`, our `RelayUrl`, and any direct
        // addresses.
        let me = endpoint.node_addr().await?;
        let nodes = vec![me];
        Ticket { topic, nodes }
    };
    println!("--> ticket to join us: {ticket}");

    // join the gossip topic by connecting to known nodes, if any
    let node_ids = nodes.iter().map(|p| p.node_id).collect();

    if nodes.is_empty() {
        println!("--> waiting for nodes to join us...");
    } else {
        println!("--> trying to connect to {} nodes...", nodes.len());
        // add the peer addrs from the ticket to our endpoint's addressbook so that they can be
        // dialed
        for node in nodes.into_iter() {
            endpoint.add_node_addr(node)?;
        }
    };

    let (sender, receiver) = gossip.subscribe_and_join(topic, node_ids).await?.split();
    println!("--> connected!");

    let message =
        Message::new(MessageBody::AboutMe { from: endpoint.node_id(), name: args.name.clone() });
    sender.broadcast(message.to_vec().into()).await?;

    tokio::spawn(subscribe_loop(endpoint.node_id(), args.name.clone(), sender.clone(), receiver));

    // spawn an input thread that reads stdin create a multi-provider, single-consumer channel
    let (line_tx, mut line_rx) = tokio::sync::mpsc::channel(1);
    // and pass the `sender` portion to the `input_loop`
    std::thread::spawn(move || input_loop(line_tx));

    // broadcast each line we type
    println!("--> type a message and hit enter to broadcast...");
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

/// Read input from stdin
fn input_loop(line_tx: tokio::sync::mpsc::Sender<String>) -> Result<()> {
    // create a new string buffer
    let mut buffer = String::new();
    // get a handle on `Stdin`
    let stdin = std::io::stdin(); // We get `Stdin` here.
    loop {
        // loop through reading from the buffer...
        stdin.read_line(&mut buffer)?;
        // and then sending over the channel
        line_tx.blocking_send(buffer.clone())?;
        // clear the buffer after we've sent the content
        buffer.clear();
    }
}

async fn subscribe_loop(
    node_id: PublicKey,
    name: String,
    sender: GossipSender,
    mut receiver: GossipReceiver,
) -> Result<()> {
    let mut names = HashMap::new();

    let me = Message::new(MessageBody::AboutMe { from: node_id, name: name.clone() });

    while let Some(event) = receiver.try_next().await? {
        if let Event::Gossip(GossipEvent::Received(msg)) = event {
            // deserialize the message and match on the message type:
            match Message::from_bytes(&msg.content)?.body {
                MessageBody::AboutMe { from, name } => {
                    // if it's an `AboutMe` message add and entry into the map and print the name
                    if !names.contains_key(&from) {
                        names.insert(from, name.clone());
                        println!("<-- {} is now known as {}", from.fmt_short(), name);
                    }

                    sender.broadcast(me.to_vec().into()).await?;
                }
                MessageBody::Message { from, text } => {
                    // if it's a `Message` message, get the name from the map and print the message
                    let name = names.get(&from).map_or_else(|| from.fmt_short(), String::to_string);
                    println!("<<< {:?}: {}", name, text.trim());
                }
            }
        } // else if Event::Lagged() {}
    }

    Ok(())
}

// add the message code to the bottom
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    body: MessageBody,
    nonce: [u8; 16],
}

#[derive(Debug, Serialize, Deserialize)]
enum MessageBody {
    AboutMe { from: NodeId, name: String },
    Message { from: NodeId, text: String },
}

impl Message {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    pub fn new(body: MessageBody) -> Self {
        Self { body, nonce: rand::random() }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Ticket {
    topic: TopicId,
    nodes: Vec<NodeAddr>,
}

impl Ticket {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = BASE32_NOPAD.encode(&self.to_bytes()[..]);
        text.make_ascii_lowercase();
        write!(f, "{}", text)
    }
}

impl FromStr for Ticket {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = BASE32_NOPAD.decode(s.to_ascii_uppercase().as_bytes())?;
        Self::from_bytes(&bytes)
    }
}
