use config::{self, Config, ConfigError};
use serde::{Deserialize, Serialize};
use std::{env, fmt};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    // read from env or set mannual
    pub version: String,
    pub threads: usize,
    pub release: bool,

    // load from yaml file or use default
    pub keep_alive: u64,
    pub database: Database,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Database {
    pub conn: String,
    pub db: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").into(),
            threads: 1,
            release: false,

            keep_alive: 60,
            database: Database::default(),
        }
    }
}

impl fmt::Display for Database {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "{}/{}", &self.conn, &self.db)
    }
}

pub fn open_yaml(fp: &str) -> Result<Settings, ConfigError> {
    let mut builder = Config::builder();

    builder = builder
        .set_default("version", "0.1.0")?
        .set_override("version", env!("CARGO_PKG_VERSION"))?
        .set_default("threads", "1")?
        .set_default("release", "false")?
        .set_default("keep_alive", "60")?
        .add_source(config::File::new(fp, config::FileFormat::Yaml));

    builder.build()?.try_deserialize::<Settings>()
}
