use actix_web::{http::StatusCode, web::Json};
use anyhow;
use serde::{self, Deserialize, Serialize};
use std::panic; // collections::HashMap
use thiserror;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("bad request: {msg}")]
    BadRequest { msg: String, cause: anyhow::Error },

    #[error("invalid parameter: {msg}")]
    InvalidParameter { msg: String, cause: anyhow::Error },

    #[error("internal server error")]
    Internal(anyhow::Error),
}

#[allow(dead_code)]
fn demo_bad_request<S: AsRef<str>>(msg: S, err: anyhow::Error) -> Error {
    Error::BadRequest { msg: msg.as_ref().to_string(), cause: err.context(func!()) }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resp<T: Serialize> {
    pub code: i16,
    pub msg: String,
    // pub data: HashMap<String, T>,
    pub data: Option<T>,
    pub request_id: String,
}
//#[serde(requestId)]
//request_id: String,

impl<T: Serialize> Resp<T> {
    pub fn new() -> Resp<T> {
        Resp {
            code: 0,
            msg: "ok".into(),
            // data: HashMap::new(),
            data: None,
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

    pub fn data(&mut self, data: T) -> &mut Self {
        self.data = Some(data);
        self
    }

    #[track_caller]
    pub fn ok(self) -> (Json<Resp<T>>, StatusCode) {
        let caller = panic::Location::caller();
        // println!("{}:{}", file!(), line!());
        println!(
            "~~~ ok: requestId={}, caller={}:{}",
            self.request_id,
            caller.file(),
            caller.line()
        );

        (Json(self), StatusCode::OK)
    }

    #[track_caller]
    pub fn bad_request(self) -> (Json<Resp<T>>, StatusCode) {
        let caller = panic::Location::caller();

        println!(
            "~~~ bad_request: requestId={}, caller={}:{}",
            self.request_id,
            caller.file(),
            caller.line()
        );

        (Json(self), StatusCode::BAD_REQUEST)
    }
}
