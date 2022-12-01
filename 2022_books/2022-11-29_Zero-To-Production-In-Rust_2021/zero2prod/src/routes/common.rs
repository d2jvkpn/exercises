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
    InternalError(anyhow::Error),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resp<T: Serialize> {
    pub code: i16,
    pub msg: String,
    // pub data: HashMap<String, T>,
    pub data: Option<T>,
    pub request_id: Uuid,
}
//#[serde(requestId)]
//request_id: String,

impl<T: Serialize> Default for Resp<T> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl<T: Serialize> Resp<T> {
    pub fn new(request_id: Option<Uuid>) -> Resp<T> {
        Resp {
            code: 0,
            msg: "ok".into(),
            // data: HashMap::new(),
            data: None,
            request_id: request_id.unwrap_or(Uuid::new_v4()),
        }
    }

    pub fn with_code(&mut self, code: i16) -> &mut Self {
        self.code = code;
        self
    }

    pub fn with_msg<S: AsRef<str>>(&mut self, msg: S) -> &mut Self {
        self.msg = msg.as_ref().to_string();
        self
    }

    pub fn with_data(&mut self, data: T) -> &mut Self {
        self.data = Some(data);
        self
    }

    #[track_caller]
    pub fn ok(self) -> (Json<Resp<T>>, StatusCode) {
        let caller = panic::Location::caller();
        println!(
            "~~~ ok: requestId={}, caller={}:{}, file!():line!()",
            self.request_id,
            caller.file(),
            caller.line(),
        );

        (Json(self), StatusCode::OK)
    }

    #[track_caller]
    pub fn bad_request(self) -> (Json<Resp<T>>, StatusCode) {
        let caller = panic::Location::caller();
        println!(
            "~~~ bad_request: requestId={}, caller={}:{}, file!():line!()",
            self.request_id,
            caller.file(),
            caller.line(),
        );

        (Json(self), StatusCode::BAD_REQUEST)
    }
}

#[cfg(test)]
mod tests {
    use super::Error::{self, BadRequest};

    fn demo_bad_request() -> Error {
        // let err = anyhow::Error::new(instance_of(std::error::Error));
        let err = anyhow::anyhow!("something is wrong");
        BadRequest { msg: "a message to frontend".to_string(), cause: err.context(func!()) }
    }

    #[test]
    fn bad_request() {
        let err = demo_bad_request();
        println!(">>> {}", err);

        if let Error::BadRequest { msg, cause } = err {
            println!(
                ">>> msg: {}, casese: {:?}",
                msg,
                cause.chain().map(|v| v.to_string()).collect::<Vec<String>>()
            );
        } else {
            panic!("not a BadRequest");
        }
    }
}
