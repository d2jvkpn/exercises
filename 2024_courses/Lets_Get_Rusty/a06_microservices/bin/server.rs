use clap::Parser;
use tonic::transport;

use std::error;

use a06::hello;

#[derive(Debug, Parser)]
#[clap(name = "server", author="d2jvkpn", version="0.1.1", long_about = None, about = "http server")]
struct Options {
    #[clap(short = 'a', long = "addr", default_value = "127.0.0.1:50001")]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let opts = Options::parse();
    dbg!(&opts);

    // let addr = "127.0.0.1:50001".parse()?;
    let addr = opts.addr.parse()?;
    eprintln!("==> GRPC server is listening on {addr:?}");

    transport::Server::builder().add_service(hello::Server::new()).serve(addr).await?;

    Ok(())
}
