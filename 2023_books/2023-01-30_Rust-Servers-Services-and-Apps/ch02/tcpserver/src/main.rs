#![allow(unused_imports)]

use std::{
    io::{Read, Write},
    net::TcpListener,
    str,
};

fn main() {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).unwrap();
    println!("=== Listening on {addr:?}");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(v) => v,
            Err(e) => {
                println!("!!! stream error: {e:?}");
                continue;
            }
        };

        println!(">>> Connection established with {:?}", stream);
        // stream.local_addr(), stream.peer_addr();

        let mut buffer = [0; 1024];
        if let Err(e) = stream.read(&mut buffer) {
            println!("!!! stream read: {e:?}");
            continue;
        }
        // String::from_utf8_lossy(&buffer)
        let result = buffer.iter().position(|v| *v == '\0' as u8);
        if let Some(v) = result {
            // let msg = str::from_utf8(&buffer[..v]).unwrap();
            let msg = String::from_utf8_lossy(&buffer[..v]);
            println!("~~~ READ: {msg:?}");
        }

        let bts = "Hello from the server".as_bytes();
        if let Err(e) = stream.write(&bts) {
            println!("!!! stream write: {e:?}");
            continue;
        }
    }
}
