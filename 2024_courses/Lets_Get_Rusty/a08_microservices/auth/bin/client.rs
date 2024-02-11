pub mod hello {
    tonic::include_proto!("hello");
}
use hello::{greeter_client::GreeterClient, HelloRequest};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:50001").await?;

    let request = tonic::Request::new(HelloRequest { name: "Tonic".into() });

    let response = client.say_hello(request).await?;
    let response = response.into_inner();

    println!("==> Got response: {response:?}");

    Ok(())
}
