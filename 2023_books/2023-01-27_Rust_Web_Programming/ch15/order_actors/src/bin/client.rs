use chrono::{Local, SecondsFormat};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

fn now() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let addr = "127.0.0.1:8080";

    let mut stream = TcpStream::connect(addr).await?;
    println!("==> {} stream starting: {addr}", now());

    let (reader, mut writer) = stream.split();

    // stream.write_all(b"Hello, world!").await?;
    writer
        .write_all(b"Buy;8.0;BYND;\nBuy;5.0;PLTR\nBuy;8.0;BYND\nBuy;5.0;PLTR\nGet\nEND\n")
        .await?;

    let mut buf_reader = BufReader::new(reader);
    let mut buf = vec![];

    let _ = buf_reader.read_until(b'\n', &mut buf).await.unwrap();

    println!("<-- recv state: {}", String::from_utf8_lossy(&buf));
    println!("<== {} stream finished", now());

    Ok(())
}
