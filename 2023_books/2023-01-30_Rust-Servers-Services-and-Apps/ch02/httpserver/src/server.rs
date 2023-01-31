use super::router::Router;
use http::HttpRequest;
use std::{io::prelude::*, net::TcpListener, str};

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("=== Listening on {:?}", self.socket_addr);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!(">>> Connection established: {stream:?}");

            let mut read_buffer = [0; 90];
            stream.read(&mut read_buffer).unwrap();

            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();

            Router::route(req, &mut stream);
        }
    }
}
