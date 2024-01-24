use chrono::{Local, SecondsFormat};
use env_logger::{Builder, Env};
use log::{debug, error, info};

use std::io::Write;

fn main() {
    println!("Hello, world!");

    eprintln!("!!! eprintln log to stderr");
    dbg!("dbg! 42");

    // std::env::set_var("RUST_LOG", "info");
    // env_logger::init();
    Builder::from_env(Env::default().default_filter_or("debug"))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}]\t{}",
                // Local::now().format("%Y-%m-%dT%H:%M:%S%:z"),
                Local::now().to_rfc3339_opts(SecondsFormat::Millis, true),
                record.level(),
                record.args(),
            )
        })
        // .filter(None, log::LevelFilter::Info)
        .init();

    info!("This is an informational message.");
    debug!("This is a debug message.");
    error!("This is an error message.");
}
