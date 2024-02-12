use clap::Parser;
use tonic::transport::Server;

use auth::hello;

use std::error::Error;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'a', long = "addr", default_value = "127.0.0.1:50001")]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    dbg!(&args);

    // let addr = "127.0.0.1:50001".parse()?;
    let addr = args.addr.parse()?;
    eprintln!("==> grpc server is listening on {addr:?}");

    Server::builder().add_service(hello::Server::new()).serve(addr).await?;

    Ok(())
}
