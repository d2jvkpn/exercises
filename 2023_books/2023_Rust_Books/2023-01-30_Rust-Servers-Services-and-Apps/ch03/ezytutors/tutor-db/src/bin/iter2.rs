use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::{env, io};

#[path = "../iter2/db_access.rs"]
mod db_access;
#[path = "../iter2/handlers.rs"]
mod handlers;
#[path = "../iter2/models.rs"]
mod models;
#[path = "../iter2/routes.rs"]
mod routes;
#[path = "../iter2/state.rs"]
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
    let app = move || App::new().app_data(app_data.clone()).configure(routes::route);
    HttpServer::new(app).bind(addr)?.run().await
}
