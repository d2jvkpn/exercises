use actix_web::{web, App, HttpServer};
use std::io;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:3000";

    let share_data = web::Data::new(AppState::default());

    println!("=== Http Server is listening on {addr:?}");
    let app = move || App::new().app_data(share_data.clone()).configure(routes::route);

    HttpServer::new(app).bind(addr)?.run().await
}
