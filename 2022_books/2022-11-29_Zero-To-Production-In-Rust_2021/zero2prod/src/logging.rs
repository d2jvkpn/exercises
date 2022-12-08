use chrono::{Local, SecondsFormat};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init_env_logger(level_filter: LevelFilter) {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {:?}",
                Local::now().to_rfc3339_opts(SecondsFormat::Millis, true),
                record.level(),
                record.args()
            )
        })
        .filter(None, level_filter)
        .init();
}
