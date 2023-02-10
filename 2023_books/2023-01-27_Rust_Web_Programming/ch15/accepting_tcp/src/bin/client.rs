use chrono::{Local, SecondsFormat};
use tokio::{io::AsyncWriteExt, net::TcpStream};

fn now() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let addr = "127.0.0.1:8080";

    let mut stream = TcpStream::connect(addr).await?;
    println!("==> {} stream starting: {}", now(), addr);
    stream.write_all(b"Hello, world!").await?;
    println!("<== {} stream finished", now());
    Ok(())
}
