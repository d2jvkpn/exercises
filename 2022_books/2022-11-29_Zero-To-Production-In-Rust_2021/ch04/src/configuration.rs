use config::{self, Config, ConfigError};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Settings {
    pub version: String,
    pub database: String,
}

pub fn open(yaml: &str) -> Result<Settings, ConfigError> {
    let mut builder = Config::builder();

    builder = builder
        .set_default("version", "0.1.0")?
        .set_override("version", env!("CARGO_PKG_VERSION"))?
        .add_source(config::File::new(yaml, config::FileFormat::Yaml));

    builder.build()?.try_deserialize::<Settings>()
}
