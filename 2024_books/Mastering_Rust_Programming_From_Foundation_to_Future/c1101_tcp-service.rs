use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let addr = "127.0.0.1:9001";
    println!("==> tcp service is listening on {:?}", addr);

    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => _ = thread::spawn(|| handle(stream)),
            Err(e) => eprintln!("!!! error accepting connection: {}", e),
        }
    }
}

// $ nc localhost 9001
fn handle(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    println!(
        "==> client connected: {:?}, thread_id: {:?}",
        stream.local_addr(),
        thread::current().id()
    );

    match stream.read(&mut buffer) {
        Err(e) => {
            eprintln!("!!! read error: {e}");
            return;
        }
        Ok(_) => println!("<== received request: {}", String::from_utf8_lossy(&buffer)),
    }

    if let Err(e) = stream.write_all(b"Hello, client!\n") {
        eprintln!("!!! write error: {e}");
    }
}
