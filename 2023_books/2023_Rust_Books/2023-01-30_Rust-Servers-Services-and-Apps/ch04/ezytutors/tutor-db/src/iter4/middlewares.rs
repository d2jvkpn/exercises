// https://actix.rs/docs/middleware/
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue, CONTENT_TYPE},
    middleware::ErrorHandlerResponse,
    Error, HttpMessage,
};
use chrono::{DateTime, Local, SecondsFormat};
use futures_util::future::LocalBoxFuture;
use std::future::{self, Ready};
use uuid::Uuid;

// https://github.com/actix/actix-web/discussions/2564
pub fn no_route<B>(sr: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    // http::header::CONTENT_TYPE, http::header::HeaderValue
    // sr.response_mut().headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("Error"));
    // Ok(ErrorHandlerResponse::Response(sr.map_into_left_body()))
    if sr.headers().get("content-type").is_some() {
        return Ok(ErrorHandlerResponse::Response(sr.map_into_left_body()));
    }

    let (req, mut res) = sr.into_parts(); // (HttpRequest, HttpResponse<B>)
    res.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let res = res.set_body(r#"{"code":-1,"msg":"no route"}"#.to_owned());

    let sr = ServiceResponse::new(req, res).map_into_boxed_body().map_into_right_body();

    Ok(ErrorHandlerResponse::Response(sr))
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SimpleLogger;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SimpleLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SimpleLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(SimpleLoggerMiddleware { service }))
    }
}

pub struct SimpleLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SimpleLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let start: DateTime<Local> = Local::now();
        let request_id = Uuid::new_v4();

        println!("==> SimpleLogger");
        let base = format!(
            "start: {}, method: {}, path: {:?}",
            start.to_rfc3339_opts(SecondsFormat::Millis, true),
            req.method(),
            req.path()
        );

        req.extensions_mut().insert(request_id);

        req.headers_mut().insert(
            HeaderName::from_lowercase(b"x-version").unwrap(),
            HeaderValue::from_str("0.1.2").unwrap(),
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            res.headers_mut().insert(
                HeaderName::from_lowercase(b"x-message").unwrap(),
                HeaderValue::from_str("Hello, world!").unwrap(),
            );

            let end: DateTime<Local> = Local::now();
            let elapsed = end.signed_duration_since(start).num_microseconds().unwrap_or(0);

            println!(
                "<== {}, elapsed: {:.3}ms, request_id: {}, status: {}",
                base,
                (elapsed as f64) / 1e3,
                request_id,
                &res.status().as_u16(),
            );

            Ok(res)
        })
    }
}
