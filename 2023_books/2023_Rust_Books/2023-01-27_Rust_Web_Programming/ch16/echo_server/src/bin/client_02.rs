use bincode::{deserialize, serialize};
use bytes::Bytes;
use futures::{sink::SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Decoder};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    pub ticker: String,
    pub amount: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut framed = BytesCodec::new().framed(stream); // blocking

    let message = Message { ticker: "BYND".into(), amount: 3.2 };

    {
        let msg_bin = serialize(&message).unwrap();
        framed.send(Bytes::from(msg_bin)).await.unwrap();

        let message = framed.next().await.unwrap().unwrap();
        let message = deserialize::<Message>(&message).unwrap();
        println!("{:?}", message);
    }

    {
        let msg_bin = serialize(&message).unwrap();
        framed.send(Bytes::from(msg_bin)).await.unwrap();

        let message = framed.next().await.unwrap().unwrap();
        let message = deserialize::<Message>(&message).unwrap();
        println!("{:?}", message);
    }

    Ok(())
}
