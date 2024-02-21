use clap::Parser;
use tonic::transport;

use a05::hello;

use std::error;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short = 'a', long = "addr", default_value = "127.0.0.1:50001")]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    dbg!(&args);

    // let addr = "127.0.0.1:50001".parse()?;
    let addr = args.addr.parse()?;
    eprintln!("==> GRPC server is listening on {addr:?}");

    transport::Server::builder().add_service(hello::Server::new()).serve(addr).await?;

    Ok(())
}
