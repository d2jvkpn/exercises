mod actors;
mod order_tracker;

use actors::*;
use chrono::{Local, SecondsFormat};
use order_tracker::*;
// use std::{thread, time::Duration};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::mpsc::{channel, Sender},
};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    let socket = TcpListener::bind(addr).await.unwrap();
    println!("==> Listening on: {}", addr);

    let (tx, rx) = channel::<Message>(1);
    let (tracker_tx, tracker_rx) = channel::<TrackerMessage>(1);

    tokio::spawn(async move { TrackerActor::new(tracker_rx).run().await });

    let tracker_txc = tracker_tx.clone();
    tokio::spawn(async move { OrderBookActor::new(rx, tracker_txc, 20.0).run().await });

    while let Ok((stream, peer)) = socket.accept().await {
        println!("\n~~~ Incoming connection from: {}", peer.to_string());
        let txc = tx.clone();
        let tracker_txc = tracker_tx.clone();
        tokio::spawn(async move { handle(stream, peer, txc, tracker_txc).await });
    }
}

fn now() -> String {
    let now = Local::now();
    now.to_rfc3339_opts(SecondsFormat::Millis, true)
}

async fn handle(
    mut stream: TcpStream,
    peer: SocketAddr,
    tx: Sender<Message>,
    tracker_tx: Sender<TrackerMessage>,
) {
    let peer_id = peer.to_string();
    println!("--> {} {peer_id} thread starting", now());
    // tokio::time::sleep(Duration::from_secs(3)).await;
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut buf = vec![];

    loop {
        let at = now();
        buf.clear();

        let msg = match reader.read_until(b'\n', &mut buf).await {
            Ok(0) => {
                eprintln!("~~~ {at} EOF received");
                return;
            }
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buf[..n]);
                println!("~~~ {at} Got message: {msg:?}");
                msg
            }
            Err(e) => {
                eprintln!("!!! {at} Error receiving message: {e}");
                return;
            }
        };

        let data: Vec<&str> = msg.split(";").map(|v| v.trim()).collect();
        // dbg!(&data);
        let command = data[0];

        match command {
            "Buy" => {
                if data.len() < 3 {
                    eprintln!("!!! Command {command:?}: invalid data length");
                    return;
                }

                let amount = match data[1].parse::<f32>() {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("!!! Command {command:?}: invalid amount: {e:?}");
                        return;
                    }
                };

                let ticker = data[2].to_string();

                let buy_order = BuyOrder::new(ticker, amount, tx.clone());
                // dbg!(&buy_order);
                buy_order.send().await;

                // let (send, _) = oneshot::channel();
                // let _ = tracker_tx
                //     .send(TrackerMessage { command: Order::Buy(ticker, amount), respond_to: send })
                //     .await;
            }
            "Get" => {
                let get_actor = GetTrackerActor { sender: tracker_tx.clone() };
                let state = get_actor.send().await;
                println!("--> Command {command:?}: sending back state {:?}", state);
                let _ = writer.write_all(state.as_bytes()).await;
                continue;
            }
            "END" => {
                return;
            }
            _ => {
                eprintln!("!!! Command {command:?}: invalid command");
                return;
            }
        }
    }

    // match writer.write(b"END\n").await {
    //     Ok(_) => println!("~~~ {} {peer_id} thread finishing", now()),
    //     Err(e) => println!("!!! {} {peer_id} thread finishing: {e:?}", now()),
    // }
}
