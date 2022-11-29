pub mod configuration;
pub mod routes;

use actix_web::{dev::Server, web, App, HttpServer};
use routes::{health_check, subscribe};
use sqlx::PgPool;
use std::{io, net::TcpListener};

pub fn run(listener: TcpListener, pool: PgPool) -> io::Result<Server> {
    let conn = web::Data::new(pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state
            .app_data(conn.clone())
    })
    .listen(listener)?
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
