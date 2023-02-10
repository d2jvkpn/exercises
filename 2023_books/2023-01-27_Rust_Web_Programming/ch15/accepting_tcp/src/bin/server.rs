use chrono::{Local, SecondsFormat};
// use std::{thread, time::Duration};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

fn now() -> String {
    let now = Local::now();
    now.to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    let socket = TcpListener::bind(addr).await.unwrap();
    println!("==> Listening on: {}", addr);

    while let Ok((stream, peer)) = socket.accept().await {
        println!("--> Incoming connection from: {}", peer.to_string());
        // tokio::spawn(async move {
        //     let peer_id = peer.to_string();
        //     println!("~~~ {} {} thread starting", now(), peer_id);
        //     tokio::time::sleep(Duration::from_secs(3)).await;
        //     println!("~~~ {} {} thread finishing", now(), peer_id);
        // });

        tokio::spawn(async move { handle(stream, peer).await });
    }
}

async fn handle(mut stream: TcpStream, peer: SocketAddr) {
    let peer_id = peer.to_string();
    println!("~~~ {} {} thread starting", now(), peer_id);
    // tokio::time::sleep(Duration::from_secs(3)).await;
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut buf = vec![];

    loop {
        let at = now();
        match reader.read_until(b'\n', &mut buf).await {
            Ok(0) => {
                println!("~~~ {at} EOF received");
                break;
            }
            Ok(n) => {
                println!("~~~ {at} Got message: {:?}", String::from_utf8_lossy(&buf[..n]));
                let _ = writer.write("Hello, world".as_bytes()).await;
            }
            Err(e) => {
                println!("!!! {at} Error receiving message: {e}");
                break;
            }
        }
    }

    println!("~~~ {} {} thread finishing", now(), peer_id);
}
