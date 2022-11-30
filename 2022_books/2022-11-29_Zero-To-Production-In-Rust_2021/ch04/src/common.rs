use actix_web::{http::StatusCode, web::Json};
use serde::{self, Deserialize, Serialize};
use std::{collections::HashMap, panic};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resp<T: Serialize> {
    pub code: i16,
    pub msg: String,
    pub data: HashMap<String, T>,
    pub request_id: String,
}
//#[serde(requestId)]
//request_id: String,

impl<T: Serialize> Resp<T> {
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
