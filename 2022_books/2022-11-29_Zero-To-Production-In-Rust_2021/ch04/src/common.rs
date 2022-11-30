use config::{self, Config, ConfigError};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Settings {
    pub database: String,
}

pub fn open_config(yaml: &str) -> Result<Settings, ConfigError> {
    let mut builder = Config::builder();

    builder = builder
        .set_default("default", "1")?
        .add_source(config::File::new(yaml, config::FileFormat::Yaml))
        .set_override("override", "1")?;

    builder.build()?.try_deserialize::<Settings>()
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resp<T> {
    pub code: i16,
    pub msg: String,
    pub data: HashMap<String, T>,
    pub request_id: String,
}
//#[serde(requestId)]
//request_id: String,

impl<T> Resp<T> {
    pub fn new() -> Resp<T> {
        Resp {
            code: 0,
            msg: "ok".into(),
            data: HashMap::new(),
            request_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn code(&mut self, code: i16) -> &mut Self {
        self.code = code;
        self
    }

    pub fn msg<S: AsRef<str>>(&mut self, msg: S) -> &mut Self {
        self.msg = msg.as_ref().to_string();
        self
    }

    pub fn data(&mut self, data: HashMap<String, T>) -> &mut Self {
        self.data = data;
        self
    }
}
