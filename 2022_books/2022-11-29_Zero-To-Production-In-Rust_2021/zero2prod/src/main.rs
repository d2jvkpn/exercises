// use env_logger;
use sqlx::PgPool;
use std::{io, net::TcpListener};
use structopt::StructOpt;
use zero2prod::{configuration::open_yaml, startup::run, telemetry::init_subscriber};

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
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    init_subscriber("zero2prod".into(), "info".into());

    // load configurations
    let opt = Opt::from_args();

    let mut config = open_yaml(&opt.config)
        .unwrap_or_else(|_| panic!("Failed to read configuration {}.", &opt.config));

    config.threads = opt.threads;
    config.release = opt.release;

    // run
    println!(
        ">>> HTTP listening on {}:{}, config={:?}, version={:?}",
        opt.addr, opt.port, opt.config, config.version,
    );

    let listener = TcpListener::bind(format!("{}:{}", opt.addr, opt.port))?;

    let pool = PgPool::connect(&config.database.to_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, pool, config)?.await
}

#[cfg(test)]
mod tests {
    #[test]
    fn t_plus() {
        let ans = 2 + 2;
        println!("ans = {}", ans);
        assert_eq!(ans, 4);
    }

    #[test]
    fn t_minus() {
        let ans = 4 - 2;
        println!("ans = {}", ans);
        assert_eq!(ans, 2);
    }
}
