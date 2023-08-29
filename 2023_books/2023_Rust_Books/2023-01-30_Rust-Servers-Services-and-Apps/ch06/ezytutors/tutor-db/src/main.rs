mod db;
mod handlers;
mod middlewares;
mod models;
mod response;
mod state;
mod utils;

use actix_web::{
    // dev::Service,
    http::StatusCode,
    middleware::{Compress, ErrorHandlers, NormalizePath},
    web,
    App,
    HttpServer,
};
use dotenv::dotenv;
// use futures_util::future::FutureExt;
use sqlx::postgres::PgPool;
use std::{env, io};

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
            //                println!("~~> Handler01"); // 2
            //                let result = srv.call(req);
            //                println!("<~~ Handler01"); // 3
            //                result
            //            })
            //            .wrap_fn(|req, srv| {
            //                println!("~~> Handler02"); // 1
            //                let result = srv.call(req);
            //                println!("<~~ Handler02"); // 4
            //                result
            //            })
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, no_route))
            .wrap(SimpleLogger)
            .wrap(Compress::default())
            .wrap(NormalizePath::default())
            .configure(handlers::route)
    };

    HttpServer::new(app).bind(addr)?.run().await
}
