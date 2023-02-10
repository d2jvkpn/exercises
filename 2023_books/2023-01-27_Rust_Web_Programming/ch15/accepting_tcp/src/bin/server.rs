#[path = "../order.rs"]
mod order;

use chrono::{Local, SecondsFormat};
use order::*;
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

    tokio::spawn(async move { BookActor::new(rx, 20.0).run().await });

    while let Ok((stream, peer)) = socket.accept().await {
        println!("==> Incoming connection from: {}", peer.to_string());
        let txc = tx.clone();
        tokio::spawn(async move { handle(stream, peer, txc).await });
    }
}

fn now() -> String {
    let now = Local::now();
    now.to_rfc3339_opts(SecondsFormat::Millis, true)
}

async fn handle(mut stream: TcpStream, peer: SocketAddr, tx: Sender<Message>) {
    let peer_id = peer.to_string();
    println!("~~~ {} {peer_id} thread starting", now());
    // tokio::time::sleep(Duration::from_secs(3)).await;
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut buf = vec![];

    loop {
        let txc = tx.clone();
        let at = now();
        let msg = match reader.read_until(b'\n', &mut buf).await {
            Ok(0) => {
                eprintln!("\n~~~ {at} EOF received");
                break;
            }
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buf[..n]);
                println!("\n~~~ {at} Got message: {msg:?}");
                // let _ = writer.write("Hello, world".as_bytes()).await;
                msg
            }
            Err(e) => {
                eprintln!("\n!!! {at} Error receiving message: {e}");
                break;
            }
        };

        let data: Vec<&str> = msg.split(";").map(|v| v.trim()).collect();
        if data.len() < 2 {
            eprintln!("!!! invalid data length");
            break;
        }

        let amount = match data[0].parse::<f32>() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("!!! invalid amount: {e:?}");
                break;
            }
        };

        let order_actor = BuyOrder::new(amount, data[1].into(), txc);
        order_actor.send().await;
        buf.clear();
    }

    match writer.write(b"BYE").await {
        Ok(_) => println!("~~~ {} {peer_id} thread finishing", now()),
        Err(e) => println!("!!! {} {peer_id} thread finishing: {e:?}", now()),
    }
}
