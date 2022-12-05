#![allow(dead_code)]

use actix_web::{
    dev::{Server, Service, ServiceResponse},
    http::header::{HeaderValue, CONTENT_TYPE},
    http::StatusCode,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpResponse, HttpServer, Result,
};
use futures_util::future::FutureExt;
use std::{io, time::Duration};

fn add_error_header<B>(mut res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("Error"));
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

fn demo_server(addr: &str) -> io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header))
            .wrap_fn(|req, srv| {
                println!("--> Hi from request start, method={}, path={}", req.method(), req.path());
                srv.call(req).map(|res| {
                    let status = res.as_ref().map_or(0, |v| v.status().as_u16());
                    println!("<-- Hi from response: status={}", status);
                    res
                })
            })
            .service(
                web::resource("/test")
                    .route(web::get().to(|| HttpResponse::Ok()))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
            )
    })
    .keep_alive(Duration::from_secs(60))
    .workers(4)
    .bind(addr)?
    .run();

    Ok(server)
}
