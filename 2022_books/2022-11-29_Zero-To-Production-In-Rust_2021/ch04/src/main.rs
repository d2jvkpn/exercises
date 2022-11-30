use ch04::{configuration, run};
use sqlx::PgPool;
use std::{io, net};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "zero2prod", about = "zero to production in rust")]
struct Opt {
    #[structopt(long = "config", default_value = "configs/local.yaml")]
    config: String,

    #[structopt(long = "addr", default_value = "127.0.0.1")]
    addr: String,

    #[structopt(long = "port", default_value = "8000")]
    port: u16,

    #[structopt(long)]
    release: bool,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    if !opt.release {
        println!("~~~ {:?}", opt);
    }

    let config = configuration::open(&opt.config).expect("Failed to read configuration.");

    let pool = PgPool::connect(&config.database).await.expect("Failed to connect to Postgres.");

    let listener = net::TcpListener::bind(format!("{}:{}", opt.addr, opt.port))?;

    run(listener, pool, 0)?.await
}
