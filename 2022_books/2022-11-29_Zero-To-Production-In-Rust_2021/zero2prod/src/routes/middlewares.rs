// https://actix.rs/docs/middleware/
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    Error,
};
use chrono::{DateTime, Local, SecondsFormat};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

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
        ready(Ok(SimpleLoggerMiddleware { service }))
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

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let start: DateTime<Local> = Local::now();

        println!(
            "==> {} {} {}",
            start.to_rfc3339_opts(SecondsFormat::Millis, true),
            req.method(),
            req.path(),
        );

        req.headers_mut().insert(
            HeaderName::from_lowercase(b"z-log-version").unwrap(),
            HeaderValue::from_str("0.1.0").unwrap(),
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let end: DateTime<Local> = Local::now();
            let elapsed = end.signed_duration_since(start).num_microseconds().unwrap_or(0);

            println!(
                "<== elapsed: {:.3}ms, status: {}",
                (elapsed as f64) / 1e3,
                res.status().as_u16(),
            );

            Ok(res)
        })
    }
}
