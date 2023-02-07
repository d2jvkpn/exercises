use actix_web::{
    dev::Service,
    http::StatusCode,
    middleware::{Compress, ErrorHandlers, NormalizePath},
    web, App, HttpServer,
};
use chrono::{Local, SecondsFormat};
use dotenv::dotenv;
// use futures_util::future::FutureExt;
use sqlx::postgres::PgPool;
use std::{env, io};

#[path = "../iter4/db_access.rs"]
mod db_access;
#[path = "../iter4/handlers.rs"]
mod handlers;
#[path = "../iter4/middlewares.rs"]
mod middlewares;
#[path = "../iter4/misc.rs"]
mod misc;
#[path = "../iter4/models.rs"]
mod models;
#[path = "../iter4/response.rs"]
mod response;
#[path = "../iter4/routes.rs"]
mod routes;
#[path = "../iter4/state.rs"]
mod state;

use middlewares::{no_route, SimpleLogger};
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is unset");
    let db = PgPool::connect(&database_url).await.unwrap();
    let app_data = web::Data::new(AppState::new(db));

    let addr = "127.0.0.1:3000";

    println!("=== Http Server is listening on {addr:?}");
    let app = move || {
        App::new()
            .app_data(app_data.clone())
            //            .wrap_fn(|req, srv| {
            //                let a01 = format!("method={}, path={}", req.method(), req.path());
            //                srv.call(req).map(move |res| {
            //                    let status = res.as_ref().map_or(0, |v| v.status().as_u16());
            //                    println!("~~~ {} response: {a01}, status={status}", now());
            //                    res
            //                })
            //            })
            //            .wrap_fn(|req, srv| {
            //                let future = srv.call(req);
            //                async {
            //                    let err = match future.await {
            //                        Ok(v) => return Ok(v),
            //                        Err(e) => e,
            //                    };
            //                    eprintln!("!!! {} error: {err:?}", now());
            //                    Err(err)
            //                }
            //            })
            .wrap_fn(|req, srv| {
                println!("~~> Handler01"); // 2
                let result = srv.call(req);
                println!("<~~ Handler01"); // 3
                result
            })
            .wrap_fn(|req, srv| {
                println!("~~> Handler02"); // 1
                let result = srv.call(req);
                println!("<~~ Handler02"); // 4
                result
            })
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, no_route))
            .wrap(SimpleLogger)
            .wrap(Compress::default())
            .wrap(NormalizePath::default())
            .configure(routes::route)
    };

    HttpServer::new(app).bind(addr)?.run().await
}

#[allow(dead_code)]
fn now() -> String {
    format!("{}", Local::now().to_rfc3339_opts(SecondsFormat::Millis, true))
}
