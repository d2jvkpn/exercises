use actix_web::{dev::Service, middleware, web, App, HttpServer};
use dotenv::dotenv;
use futures_util::future::FutureExt;
use sqlx::postgres::PgPool;
use std::{env, io};

#[path = "../iter4/db_access.rs"]
mod db_access;
#[path = "../iter4/handlers.rs"]
mod handlers;
#[path = "../iter4/models.rs"]
mod models;
#[path = "../iter4/response.rs"]
mod response;
#[path = "../iter4/routes.rs"]
mod routes;
#[path = "../iter4/state.rs"]
mod state;

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
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::default())
            .wrap_fn(|req, srv| {
                let a01 = format!("method={}, path={}", req.method(), req.path());
                srv.call(req).map(move |res| {
                    let status = res.as_ref().map_or(0, |v| v.status().as_u16());
                    println!("~~~ response: {a01}, status={}", status);
                    res
                })
            })
            .wrap_fn(|req, srv| {
                let future = srv.call(req);
                async {
                    let err = match future.await {
                        Ok(v) => return Ok(v),
                        Err(e) => e,
                    };
                    eprintln!("!!! {err:?}");
                    Err(err)
                }
            })
            .configure(routes::route)
    };

    HttpServer::new(app).bind(addr)?.run().await
}
