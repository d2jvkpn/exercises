use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use serde_json::json;
use sqlx::error::Error as SQLxError;
// use std::fmt;
use thiserror;

#[allow(dead_code)]
pub enum Result {
    Ok(HttpResponse),
    Err(Error),
}
// from std::result::Result<T, Error> -> Result

//#[derive(Serialize)]
//pub struct ResponseOk<T> {
//    pub code: i32,
//    pub msg: String,
//    pub data: Option<T>,
//}

//impl<T> From<Data<T>> for ResponseOk<T> {
//    fn from(v: Data<T>) -> Self {
//        Self { code: 0, msg: "ok".into(), data: Some(v.0) }
//    }
//}

// response data
#[derive(Debug, Serialize)]
pub struct Data<T>(pub T);

impl<T: Serialize> From<Data<T>> for HttpResponse {
    fn from(v: Data<T>) -> Self {
        HttpResponse::Ok().json(json!({"code": 0, "msg": "ok", "data": v}))
    }
}

// response error
// TODO: log error, using thiserror
#[allow(dead_code)]
#[derive(Debug, Serialize, thiserror::Error)]
pub enum Error {
    // -1
    #[error("no route")]
    NoRoute,

    // 1
    #[error("canceled: {0}")]
    Canceled(String),

    // 2
    #[error("unknown")]
    Unknown,

    // 3
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    // 5
    #[error("not found: {0}")]
    NotFound(String),

    // 7
    #[error("permission denied")]
    PermissionDenied,

    // 8
    #[error("resource exhausted")]
    ResourceExhausted,

    // 10
    #[error("aboort")]
    Aborted,

    // 13
    #[error("database error")]
    DBError(String),

    // 13
    #[error("internal server error")]
    ActixError(String),

    // 16
    #[error("unauthenticated")]
    Unauthenticated,
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
            Self::Canceled(_) => 1,
            Self::Unknown => 2,
            Self::InvalidArgument(_) => 3,
            Self::NotFound(_) => 5,
            Self::PermissionDenied => 7,
            Self::ResourceExhausted => 8,
            Self::Aborted => 10,
            Self::DBError(_) | Self::ActixError(_) => 13,
            Self::Unauthenticated => 16,
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NoRoute => StatusCode::BAD_REQUEST,
            Self::Canceled(_) => StatusCode::NOT_ACCEPTABLE,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::PermissionDenied => StatusCode::FORBIDDEN,
            Self::ResourceExhausted => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Aborted => StatusCode::NOT_ACCEPTABLE,
            Self::DBError(_) | Self::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthenticated => StatusCode::UNAUTHORIZED,
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
