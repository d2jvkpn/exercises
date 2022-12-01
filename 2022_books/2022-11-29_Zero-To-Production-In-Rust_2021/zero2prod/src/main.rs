use sqlx::PgPool;
use std::{io, net};
use structopt::StructOpt;
use zero2prod::{configuration, run};

#[allow(dead_code)]
#[derive(Debug, StructOpt)]
#[structopt(name = "zero2prod", about = "zero to production in rust")]
struct Opt {
    #[structopt(long = "config", default_value = "configs/local.yaml")]
    config: String,

    #[structopt(long = "addr", default_value = "0.0.0.0")]
    addr: String,

    #[structopt(long = "port", default_value = "8000")]
    port: u16,

    #[structopt(long = "threads", default_value = "0")]
    threads: usize,

    #[structopt(long)]
    release: bool,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let config = configuration::open(&opt.config).expect("Failed to read configuration.");

    println!(
        ">>> HTTP listening on {}:{}, config={:?}, version={:?}",
        opt.addr, opt.port, opt.config, config.version,
    );

    let listener = net::TcpListener::bind(format!("{}:{}", opt.addr, opt.port))?;
    let pool = PgPool::connect(&config.database).await.expect("Failed to connect to Postgres.");

    run(listener, pool, opt.threads)?.await
}
