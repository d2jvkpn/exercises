use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::thread;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";

    println!(">>> HTTP Server is listening on {:?}", addr);

    HttpServer::new(|| {
        println!("    http server factory is firing: {:?}", thread::current().id());
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/say/hello", web::get().to(|| async { "Hello Again!" }))
    })
    .workers(3)
    .bind(addr)?
    // .bind("127.0.0.1:8081")? // can bind multiple ports
    .run()
    .await
}
