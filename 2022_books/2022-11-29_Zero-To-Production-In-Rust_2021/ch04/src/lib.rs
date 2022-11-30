pub mod configuration;
pub mod routes;

use actix_web::{
    dev::Server, get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder,
};
use routes::subscribe;
use sqlx::PgPool;
use std::{io, net, thread};

pub fn run(listener: net::TcpListener, pool: PgPool, mut workers: usize) -> io::Result<Server> {
    let conn = web::Data::new(pool);

    let threads = thread::available_parallelism().unwrap().get();
    if workers == 0 || workers > threads {
        workers = threads;
    }

    let server = HttpServer::new(move || {
        println!("~~~ start http server");

        App::new()
            .route("/health", web::get().to(health_check))
            .service(healthy)
            .route("/open/subscribe", web::post().to(subscribe))
            // Register the connection as part of the application state
            .app_data(conn.clone())
    })
    .listen(listener)?
    .workers(workers)
    .run();

    Ok(server)
}

pub async fn health_check() -> HttpResponse {
    // return "impl Responder" is ok too
    HttpResponse::Ok().finish()
}

#[get("/healthy")]
pub async fn healthy() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Version", "0.1.0"))
        .body("Ok")
}

#[cfg(test)]
mod tests {
    use super::health_check;

    #[actix_rt::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}
