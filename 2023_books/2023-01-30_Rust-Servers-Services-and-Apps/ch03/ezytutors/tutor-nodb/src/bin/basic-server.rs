use actix_web::{
    web::{get, ServiceConfig},
    App, HttpResponse, HttpServer, Responder,
};
use std::io;

pub fn general_routes(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello, EzyTutor i alive an kicking")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:3000";

    let app = move || App::new().configure(general_routes);

    println!("=== Http server is listening on {addr:?}");
    HttpServer::new(app).bind(addr)?.run().await
}
