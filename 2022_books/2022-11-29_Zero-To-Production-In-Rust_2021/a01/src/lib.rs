use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::io;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!\n", &name)
}

async fn health_check() -> HttpResponse {
    // return "impl Responder" is ok too
    HttpResponse::Ok().finish()
}

pub fn run(addr: &str) -> Result<Server, io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/greet/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind(addr)?
    .run();

    Ok(server)
}

#[cfg(test)]
mod tests {
    use crate::health_check;
    #[actix_rt::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        // This requires changing the return type of `health_check`
        // from `impl Responder` to `HttpResponse` to compile
        // You also need to import it with `use actix_web::HttpResponse`!
        assert!(response.status().is_success())
    }
}
