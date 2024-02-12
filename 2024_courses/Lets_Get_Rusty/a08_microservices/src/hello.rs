use tonic::{Request, Response, Status};

pub mod hello_proto {
    tonic::include_proto!("hello"); // grpc package name
}
use hello_proto::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

#[derive(Debug)]
pub struct Server {}

#[tonic::async_trait]
impl Greeter for Server {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("==> Got a request: {:?}", request);

        // We must use .into_inner() as the fields of gRPC requests and responses are private
        let reply = HelloReply { message: format!("Hello {}!", request.into_inner().name) };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

impl Server {
    pub fn new() -> GreeterServer<Self> {
        GreeterServer::new(Server {})
    }
}
