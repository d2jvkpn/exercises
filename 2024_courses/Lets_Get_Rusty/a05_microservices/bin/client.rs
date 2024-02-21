use clap::Parser;
use tonic::Request;

pub mod hello_proto {
    tonic::include_proto!("hello");
}
use hello_proto::{greeter_client::GreeterClient, HelloRequest};

use std::error;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short = 'a', long = "addr", default_value = "http://127.0.0.1:50001")]
    addr: String,

    #[clap(short = 'n', long = "name", default_value = "Tonic")]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    dbg!(&args);

    let mut client = GreeterClient::connect(args.addr).await?;

    let request = Request::new(HelloRequest { name: args.name });

    let response = client.say_hello(request).await?;
    let response = response.into_inner();

    println!("==> Got response: {response:?}");

    Ok(())
}
