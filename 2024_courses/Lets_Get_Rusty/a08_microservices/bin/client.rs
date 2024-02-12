use tonic::Request;

pub mod hello_proto {
    tonic::include_proto!("hello");
}
use hello_proto::{greeter_client::GreeterClient, HelloRequest};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:50001").await?;

    let request = Request::new(HelloRequest { name: "Tonic".into() });

    let response = client.say_hello(request).await?;
    let response = response.into_inner();

    println!("==> Got response: {response:?}");

    Ok(())
}
