use tonic::{transport::Server, Request, Response, Status};

use std::error;

pub mod hello {
    tonic::include_proto!("hello"); // grpc package name
}
use hello::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("==> Got a request: {:?}", request);

        let reply = hello::HelloReply {
            // We must use .into_inner() as the fields of gRPC requests and responses are private
            msg: format!("Hello {}!", request.into_inner().name),
            timestamp: "".to_string(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let addr = "127.0.0.1:50001".parse()?;
    let greeter: MyGreeter = Default::default();

    eprintln!("==> grpc server is listening on {addr:?}");

    Server::builder().add_service(GreeterServer::new(greeter)).serve(addr).await?;

    Ok(())
}
