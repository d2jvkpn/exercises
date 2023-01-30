use std::{
    io::{Read, Write},
    net::TcpStream,
    process, str,
};

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 1024];

    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("!!! stream read: {e:?}");
        process::exit(1);
    };

    let result = buffer.iter().position(|v| *v == '\0' as u8);
    if let Some(v) = result {
        let msg = str::from_utf8(&buffer[..v]).unwrap();
        println!(">>> Got response from server: {:?}", msg);
    } else {
        println!("!!! Response nothing")
    }
}
