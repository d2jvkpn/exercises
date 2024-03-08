use clap::Parser;
use tonic::Request;

use std::error;

pub mod hello_proto {
    tonic::include_proto!("hello");
}
use hello_proto::{greeter_client::GreeterClient, HelloRequest};

#[derive(Debug, Parser)]
#[clap(name = "client", author="d2jvkpn", version="0.1.1", long_about = None, about = "http request client")]
struct Options {
    #[clap(short = 'a', long = "addr", default_value = "http://127.0.0.1:50001")]
    addr: String,

    #[clap(short = 'n', long = "name", default_value = "Tonic")]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let opts = Options::parse();
    dbg!(&opts);

    let mut client = GreeterClient::connect(opts.addr).await?;

    let request = Request::new(HelloRequest { name: opts.name });

    let response = client.say_hello(request).await?;
    let response = response.into_inner();

    println!("==> Got response: {response:?}");

    Ok(())
}
