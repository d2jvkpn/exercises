use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::console::{ConsoleAppender, Target},
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::json::JsonEncoder,
    // encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use std::thread;

fn main() -> Result<(), SetLoggerError> {
    let level = log::LevelFilter::Info;
    let file_path = "logs/log4rs.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(JsonEncoder::new()))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?;

    let h1 = thread::spawn(|| {
        log_mdc::insert("foo", "bar");
        error!(target:"yak_events", "H1: Goes to stderr and file");
        warn!("H1: Goes to stderr and file");
    });

    let h2 = thread::spawn(|| {
        info!("H2: Goes to stderr and file");
        debug!("H2: Goes to file only");
        trace!("H2: Goes to file only");
    });

    h1.join().unwrap();
    h2.join().unwrap();

    Ok(())
}
