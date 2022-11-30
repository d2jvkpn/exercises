pub mod configuration;
pub mod routes;

use actix_web::{dev::Server, web, App, HttpServer};
use routes::{health_check, subscribe};
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
            .route("/subscribe", web::post().to(subscribe))
            // Register the connection as part of the application state
            .app_data(conn.clone())
    })
    .listen(listener)?
    .workers(workers)
    .run();

    Ok(server)
}

#[cfg(test)]
mod tests {
    use crate::routes::health_check;
    #[actix_rt::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}
