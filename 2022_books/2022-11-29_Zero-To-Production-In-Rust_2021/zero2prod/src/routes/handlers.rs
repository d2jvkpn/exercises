/*!
Status Codes (go doc http.StatusOK):
- 200 StatusOK
- 400 StatusBadRequest *
- 401 StatusUnauthorized
- 404 StatusNotFound *
- 409 StatusConflict
- 426 StatusUpgradeRequired
- 429 StatusTooManyRequests
- 500 StatusInternalServerError *
- 503 StatusServiceUnavailable
- 505 StatusHTTPVersionNotSupported
!*/

use actix_web::{
    dev::ServiceResponse,
    http::header::{HeaderValue, CONTENT_TYPE},
    middleware::ErrorHandlerResponse,
};

// https://github.com/actix/actix-web/discussions/2564
pub fn render_400<B>(sr: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    // http::header::CONTENT_TYPE, http::header::HeaderValue
    // sr.response_mut().headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("Error"));
    // Ok(ErrorHandlerResponse::Response(sr.map_into_left_body()))
    if sr.headers().get("content-type").is_some() {
        return Ok(ErrorHandlerResponse::Response(sr.map_into_left_body()));
    }

    let (req, mut res) = sr.into_parts(); // (HttpRequest, HttpResponse<B>)
    res.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let res = res.set_body(r#"{"code":-1,"msg":"bad request"}"#.to_owned());

    let sr = ServiceResponse::new(req, res).map_into_boxed_body().map_into_right_body();

    Ok(ErrorHandlerResponse::Response(sr))
}

pub fn render_404<B>(sr: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    if sr.headers().get("content-type").is_some() {
        return Ok(ErrorHandlerResponse::Response(sr.map_into_left_body()));
    }

    let (req, mut res) = sr.into_parts();

    res.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let res = res.set_body(r#"{"code":-2,"msg":"no route"}"#.to_owned());

    let sr = ServiceResponse::new(req, res).map_into_boxed_body().map_into_right_body();

    Ok(ErrorHandlerResponse::Response(sr))
}

pub fn render_500<B>(sr: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    if sr.headers().get("content-type").is_some() {
        return Ok(ErrorHandlerResponse::Response(sr.map_into_left_body()));
    }

    let (req, mut res) = sr.into_parts();
    res.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let res = res.set_body(r#"{"code":1,"msg":"internal server error"}"#.to_owned());

    let sr = ServiceResponse::new(req, res).map_into_boxed_body().map_into_right_body();

    Ok(ErrorHandlerResponse::Response(sr))
}
