use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use serde_json::json;
use sqlx::error::Error as SQLxError;
// use std::fmt;
use thiserror;

#[derive(Serialize)]
pub struct ResponseOk<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> From<Data<T>> for ResponseOk<T> {
    fn from(v: Data<T>) -> Self {
        Self { code: 0, msg: "ok".into(), data: Some(v.0) }
    }
}

// response data
#[derive(Debug, Serialize)]
pub struct Data<T>(pub T);

impl<T: Serialize> From<Data<T>> for HttpResponse {
    fn from(v: Data<T>) -> Self {
        HttpResponse::Ok().json(ResponseOk::from(v))
    }
}

// response error
// TODO: log error, using thiserror
#[allow(dead_code)]
#[derive(Debug, Serialize, thiserror::Error)]
pub enum Error {
    #[error("no route")]
    NoRoute,
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("database error")]
    DBError(String),
    #[error("internal server error")]
    ActixError(String),
}

//impl fmt::Display for Error {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", self)
//    }
//}

impl Error {
    //    fn msg(&self) -> String {
    //        match self {
    //            Self::NoRoute => {
    //                eprintln!("No route");
    //                "No route".into()
    //            }
    //            Self::InvalidArgument(v) => {
    //                eprintln!("invalid argument: {v:?}");
    //                "Invalid argument".into()
    //            }
    //            Self::NotFound(v) => {
    //                eprintln!("Not found occurred: {v:?}");
    //                format!("Not found: {v}")
    //            }
    //            Self::DBError(v) => {
    //                eprintln!("Database error occurred: {v:?}");
    //                "Database error".into()
    //            }
    //            Self::ActixError(v) => {
    //                eprintln!("Server error occurred: {v:?}");
    //                "Internal server error".into()
    //            }
    //        }
    //    }

    fn code(&self) -> i32 {
        match self {
            Self::NoRoute => -1,
            Self::InvalidArgument(_) => -2,
            Self::NotFound(_) => -3,
            Self::DBError(_) => 2,
            Self::ActixError(_) => 1,
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NoRoute | Self::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::DBError(_) | Self::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(json!({"code": self.code(),"msg": format!("{}", self)}))
    }
}

impl From<actix_web::error::Error> for Error {
    fn from(err: actix_web::error::Error) -> Self {
        Self::ActixError(err.to_string())
    }
}

impl From<SQLxError> for Error {
    fn from(err: SQLxError) -> Self {
        Self::DBError(err.to_string())
    }
}
