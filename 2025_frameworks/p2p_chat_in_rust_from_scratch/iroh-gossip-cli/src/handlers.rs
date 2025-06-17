use std::collections::HashMap;

use crate::structs::{Message, MessageBody, QUIT};

use anyhow::Result;
use futures_lite::StreamExt;
use iroh::PublicKey;
use iroh_gossip::net::{Event, GossipEvent, GossipReceiver, GossipSender};
use tokio::sync::mpsc;

/// Read input from stdin
pub fn input_loop(line_tx: mpsc::Sender<String>) -> Result<()> {
    // create a new string buffer
    let mut buffer = String::new();
    let eol = &['\r', '\n'][..];
    // get a handle on `Stdin`
    let stdin = std::io::stdin(); // We get `Stdin` here.

    loop {
        stdin.read_line(&mut buffer)?; // loop through reading from the buffer...
        // let line = buffer.trim_end().to_string();
        if buffer.trim_end_matches(eol).ends_with(' ') {
            buffer.truncate(buffer.trim_end().len());
            buffer.push('\n');
            continue;
        }

        let line = buffer.trim_end();
        let quit = line == QUIT;
        line_tx.blocking_send(line.to_string())?; // and then sending over the channel
        buffer.clear(); // clear the buffer after we've sent the content

        if quit {
            break Ok(());
        }
    }
}

pub async fn subscribe_loop(
    node_id: PublicKey,
    name: String,
    sender: GossipSender,
    mut receiver: GossipReceiver,
) -> Result<()> {
    let mut members = HashMap::new();
    let abount_me = Message::new(MessageBody::AboutMe { from: node_id, name: name.to_string() });

    while let Some(event) = receiver.try_next().await? {
        let msg = match event {
            Event::Gossip(GossipEvent::Received(msg)) => msg,
            Event::Gossip(GossipEvent::NeighborDown(msg)) => {
                println!("--> NeighborDown: {msg:?}");
                members.remove_entry(&msg);
                continue;
            }
            Event::Gossip(GossipEvent::NeighborUp(msg)) => {
                println!("--> NeighborUp: {msg:?}");
                continue;
            }
            Event::Gossip(GossipEvent::Joined(msg)) => {
                println!("--> Joined: {msg:?}");
                continue;
            }
            Event::Lagged => {
                println!("--> Lagged");
                continue;
            }
        };

        // deserialize the message and match on the message type:
        match Message::from_bytes(&msg.content)?.body {
            MessageBody::Bye { from } => match members.remove_entry(&from) {
                Some((node_id, name)) => println!("--> Bye: {node_id}, {name:?}"),
                None => println!("--> Bye: {from}, ????"),
            },
            MessageBody::AboutMe { from, name } => {
                // if it's an `AboutMe` message add and entry into the map and print the name
                if !members.contains_key(&from) {
                    members.insert(from, name.clone());
                    println!("<-- {} is now known as {:?}", from.fmt_short(), name);
                }

                if let Err(e) = sender.broadcast(abount_me.to_vec().into()).await {
                    println!("!!! broadcast error: {e:?}");
                }
            }
            MessageBody::Message { from, text } => {
                // if it's a `Message` message, get the name from the map and print the message
                let name = members.get(&from).map_or_else(|| from.fmt_short(), String::to_string);
                println!("<<< {:?}: {}", name, text.trim_end());
            }
        }
    }

    Ok(())
}
