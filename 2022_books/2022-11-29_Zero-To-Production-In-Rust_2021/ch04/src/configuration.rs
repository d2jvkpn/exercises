use config::{self, Config, ConfigError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: String,
}

pub fn open(yaml: &str) -> Result<Settings, ConfigError> {
    let mut builder = Config::builder();

    builder = builder
        .set_default("default", "1")?
        .add_source(config::File::new(yaml, config::FileFormat::Yaml))
        .set_override("override", "1")?;

    builder.build()?.try_deserialize::<Settings>()
}
