use chrono::{Local, SecondsFormat};

pub fn now() -> String {
    format!(
        "{}",
        Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
    )
}
