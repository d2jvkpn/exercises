#![allow(dead_code)]

use std::net::SocketAddr;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

async fn handle_chat(stream: &mut TcpStream, _peer: SocketAddr) -> Result<(), String> {
    // let mut buffer = [0; 1024];
    let mut buffer = Vec::with_capacity(1024);

    let size = stream.read(&mut buffer).await.map_err(|e| format!("read error: {e}"))?;
    // size > 0 && (buffer[size-1] == b'\n' || buffer[0] == b'\r')
    if size == 0 {
        return Err("EOF".into());
    }

    // String::from_utf8_lossy(&buffer[..size])
    match String::from_utf8((&buffer[..size]).to_vec()) {
        Ok(v) => print!("~~~ Read {} bytes: {}", size, v),
        Err(e) => return Err(format!("not uft8: {e}")),
    };

    stream.write(&buffer[0..size]).await.map_err(|e| format!("write error: {e}"))?;
    Ok(())
}

async fn handle_stream(mut stream: TcpStream, _peer: SocketAddr) -> Result<(), std::io::Error> {
    let mut buffer = vec![0u8; 1024];
    let size = stream.read(&mut buffer).await?;

    // let mut msg = String::with_capacity(1024);
    // let result = stream.read_to_string(&mut msg).await?;
    println!("<-- Got message(size={}): {})", size, String::from_utf8_lossy(&buffer[..size]));

    stream.write_all("HTTP/1.1 200 Ok\n\nHello, world!\n".as_bytes()).await?;
    Ok(())
}

async fn handle_lines(mut stream: TcpStream, _peer: SocketAddr) {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);

    let mut buf = Vec::with_capacity(1024);

    loop {
        buf.clear();
        let size = match reader.read_until(b'\n', &mut buf).await {
            Ok(0) => return,
            Ok(v) => {
                println!("~~~ Got: {:?}", &buf[..v]);
                v
            }
            Err(e) => {
                eprintln!("!!! Read error: {e:?}");
                return;
            }
        };

        if let Err(e) = writer.write_all(&buf[..size]).await {
            eprintln!("!!! Write error: {e:?}");
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn chars() {
        println!("~~~~ {:?}, {}", 10 as char, b'\r' as u8); // \n: q0, \r: 13
    }
}
