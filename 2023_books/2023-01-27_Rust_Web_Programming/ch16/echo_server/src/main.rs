use bincode::{deserialize, serialize};
use bytes::Bytes;
use futures::{sink::SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{BytesCodec, Decoder};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Message {
    pub ticker: String,
    pub amount: f32,
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("==> Listening on: {}", addr);

    while let Ok((stream, peer)) = listener.accept().await {
        tokio::spawn(async move { handle(stream, peer).await });
    }
}

async fn handle(stream: TcpStream, peer: SocketAddr) {
    println!("--> Incoming connection from: {}", peer.to_string());
    let mut framed = BytesCodec::new().framed(stream);

    let result = loop {
        let msg = match framed.next().await {
            Some(v) => v,
            None => break Ok(()),
        };

        let bts = match msg {
            Ok(v) => v,
            Err(e) => break Err(e),
        };

        let message = deserialize::<Message>(&bts).unwrap();
        println!("<~~ Got message: {message:?}");
        let message_bin = serialize(&message).unwrap();

        if let Err(e) = framed.send(Bytes::from(message_bin)).await {
            break Err(e);
        };
    };

    match result {
        Ok(_) => println!("<-- Socket received FIN packet and closed connection"),
        Err(e) => println!("!!! Socket closed with error: {e:?}"),
    }
}
