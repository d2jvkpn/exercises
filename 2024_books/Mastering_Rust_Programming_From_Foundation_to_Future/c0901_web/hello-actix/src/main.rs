use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;

use std::io;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    pub name: String,
}

#[get("/hello")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/hello/{name}")]
async fn hello_name(name: web::Path<String>) -> impl Responder {
    let templ = HelloTemplate { name: name.to_string() };
    HttpResponse::Ok().body(templ.render().unwrap())
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080";

    println!("==> http service is listening on {addr:?}");
    HttpServer::new(|| App::new().service(hello_name)).bind(addr)?.run().await
}
