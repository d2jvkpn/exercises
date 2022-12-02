// use env_logger;
use sqlx::PgPool;
use std::{io, net::TcpListener};
use structopt::StructOpt;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::{configuration::open_yaml, startup::run};

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
    // setup tracing
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    // The `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    let subscriber =
        Registry::default().with(env_filter).with(JsonStorageLayer).with(formatting_layer);
    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");

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
