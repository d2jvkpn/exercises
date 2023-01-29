use actix_web::{
    dev::Payload,
    http::header::{self, ContentType, HeaderValue},
    http::StatusCode,
    FromRequest, HttpRequest, HttpResponse, ResponseError,
};
use derive_more::Display;
use futures::future::{self, Ready};

pub struct JwToken {
    pub message: String,
}

#[allow(dead_code)]
#[derive(Debug, Display)]
pub enum TokenError {
    #[display(fmt = "token not found")]
    TokenNotFound,

    #[display(fmt = "token expired")]
    TokenExpired,

    #[display(fmt = "invalid token")]
    InvalidToken,
}

impl ResponseError for TokenError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::TokenNotFound => {
                let mut response = HttpResponse::new(StatusCode::UNAUTHORIZED);
                let header_value = HeaderValue::from_str(r#"Basic realm="publish""#).unwrap();
                // actix_web::http::header provides a collection of constants
                // for the names of several well-known/standard HTTP headers
                response.headers_mut().insert(header::WWW_AUTHENTICATE, header_value);
                response
            }
            Self::TokenExpired => HttpResponse::Unauthorized()
                .content_type(ContentType::plaintext())
                .insert_header(("X-Hdr", "jwt"))
                .body("token expired"),
            Self::InvalidToken => HttpResponse::Unauthorized()
                .content_type(ContentType::plaintext())
                .insert_header(("X-Hdr", "jwt"))
                .body("invalid token"),
        }
    }
}

impl FromRequest for JwToken {
    type Error = TokenError;
    type Future = Ready<Result<JwToken, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => future::ok(JwToken { message: data.to_str().unwrap().to_string() }),
            None => future::err(TokenError::TokenNotFound),
        }
    }
}
