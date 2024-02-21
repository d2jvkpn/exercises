use tokio::{io::AsyncWriteExt, net::TcpStream};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // println!("Hello, wrold!");

    let mut stream = TcpStream::connect("127.0.0.1:8000").await?;

    stream.write_all(b"Hello, world!\n").await?;

    Ok(())
}
