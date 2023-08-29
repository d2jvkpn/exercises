use actix_web::{
    http::header::ContentType,
    web::{get, ServiceConfig},
    App, HttpResponse, HttpServer, Responder,
};
use std::{io, thread};

pub fn general_routes(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Version", "0.1.0"))
        .body("Hello, EzyTutor i alive an kicking")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:3000";

    let mut threads = thread::available_parallelism().unwrap().get();
    dbg!(&threads);

    threads = if threads > 1 { threads / 2 } else { threads };
    println!(
        "=== Http server is listening on {addr:?}, threads: {threads}, main thread: {:?}",
        thread::current().id()
    );

    let app = move || {
        println!("~~~ {:?}", thread::current().id());
        App::new().configure(general_routes)
    };

    HttpServer::new(app).bind(addr)?.workers(threads).run().await
}
