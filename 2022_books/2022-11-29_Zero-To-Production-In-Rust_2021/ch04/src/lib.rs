pub mod configuration;
pub mod misc;
pub mod routes;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::{io, net, thread, time};

pub fn run(listener: net::TcpListener, pool: PgPool, mut workers: usize) -> io::Result<Server> {
    let data = web::Data::new(pool);

    let threads = thread::available_parallelism().unwrap().get();
    if workers == 0 || workers > threads {
        workers = threads;
    }

    let server = HttpServer::new(move || {
        println!("~~~ start http server");

        App::new()
            .route("/health", web::get().to(misc::health_check))
            .service(misc::healthy)
            .configure(misc::load_open)
            // Register the connection as part of the application state
            .app_data(data.clone())
    })
    .keep_alive(time::Duration::from_secs(75))
    .listen(listener)?
    .workers(workers)
    .run();

    Ok(server)
}

#[cfg(test)]
mod tests {
    use crate::misc::health_check;

    #[actix_rt::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}
