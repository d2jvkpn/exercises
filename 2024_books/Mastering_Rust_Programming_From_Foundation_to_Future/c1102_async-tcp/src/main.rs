use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use std::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let addr = "127.0.0.1:9002";
    println!("==> tcp service is listening on {addr:?}");

    let listener = TcpListener::bind(addr).await?;

    while let Ok((mut stream, addr)) = listener.accept().await {
        tokio::spawn(async move {
            println!("==> client connected: {:?}", /*stream.local_addr()*/ addr);
            let mut buffer = [0; 512];

            if let Err(e) = stream.read(&mut buffer).await {
                eprintln!("Failed to read from socket: {}", e);
                return;
            }

            let response = "Hello, client!\n";
            if let Err(e) = stream.write_all(response.as_bytes()).await {
                eprintln!("Failed to write to socket: {}", e);
                return;
            }
        });
    }

    Ok(())
}

// $ nc localhost 9002
