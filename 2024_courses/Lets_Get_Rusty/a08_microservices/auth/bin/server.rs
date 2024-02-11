use tonic::transport::Server;

use auth::hello;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:50001".parse()?;
    eprintln!("==> grpc server is listening on {addr:?}");

    Server::builder().add_service(hello::Server::new()).serve(addr).await?;

    Ok(())
}
