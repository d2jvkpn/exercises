use anyhow::Error;
use log::{debug, error, info, trace, warn, LevelFilter};
use log4rs::{
    append::console::{ConsoleAppender, Target},
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::json::JsonEncoder,
    // encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use serde_json::json;
use std::{fs, io, thread};

fn main() -> anyhow::Result<()> {
    if let Err(e) = fs::remove_dir_all("logs") {
        if e.kind() != io::ErrorKind::NotFound {
            return Err(Error::new(e).context("remove logs/"));
        }
    };
    // fs::create_dir_all("logs").map_err(|e| Error::new(e).context("create logs/"))?;

    let (level, file_path) = (log::LevelFilter::Info, "logs/log4rs.log");

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(JsonEncoder::new()))
        .build(file_path)
        .map_err(|e| Error::new(e).context("log4rs create logfile from FileAppender"))?;

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
        .map_err(|e| Error::new(e).context("log4rs config builder"))?;

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?; // SetLoggerError

    let h1 = thread::spawn(|| {
        log_mdc::insert("h1", "foo-bar");
        log_mdc::insert("ans", "24");

        error!(target:"yak_events", "H1: Goes to stderr and file");
        warn!("H1: Goes to stderr and file");
    });

    let h2 = thread::spawn(|| {
        log_mdc::insert("h2", "thread2");
        log_mdc::insert("answer", json!({"ans":42}).to_string());
        log_mdc::remove("h2");

        // clear          Removes all values from the MDC.
        // extend         Extends the MDC with new entries.
        // extend_scoped  Extends the MDC with new entries in a scoped fashion.
        // get            Retrieves a value from the MDC.
        // insert         Inserts a new entry into the MDC, returning the old value.
        // insert_scoped  Inserts a new entry into the MDC in a scoped fashion.
        // iter           Invokes the provided closure for each entry in the MDC.
        // remove         Removes a value from the MDC.
        info!("H2: Goes to stderr and file");
        debug!("H2: Goes to file only");
        trace!("H2: Goes to file only");
    });

    h1.join().unwrap();
    h2.join().unwrap();
    // TODO: join multiple errors

    Ok(())
}
