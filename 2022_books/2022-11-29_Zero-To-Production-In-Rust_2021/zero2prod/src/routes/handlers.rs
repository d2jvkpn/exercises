use actix_web::{
    dev::ServiceResponse,
    http::header::{HeaderValue, CONTENT_TYPE},
    middleware::ErrorHandlerResponse,
};

// https://github.com/actix/actix-web/discussions/2564
pub fn render_404<B>(sr: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    // http::header::CONTENT_TYPE, http::header::HeaderValue
    // res.response_mut().headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("Error"));
    // Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))

    let (req, mut res) = sr.into_parts(); // (HttpRequest, HttpResponse<B>)
    res.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let res = res.set_body(r#"{"code":-1,"msg":"no route"}"#.to_owned());

    let sr = ServiceResponse::new(req, res).map_into_boxed_body().map_into_right_body();

    Ok(ErrorHandlerResponse::Response(sr))
}

pub fn render_500<B>(sr: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (req, mut res) = sr.into_parts();
    res.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let res = res.set_body(r#"{"code":1,"msg":"internal server error"}"#.to_owned());

    let sr = ServiceResponse::new(req, res).map_into_boxed_body().map_into_right_body();

    Ok(ErrorHandlerResponse::Response(sr))
}
