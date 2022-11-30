#[macro_export]
macro_rules! func {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        let caller = std::panic::Location::caller();
        let name = type_name_of(f);
        let list: Vec<&str> = name.split("::").collect();
        // println!("??? {:?}", list);
        let length = list.len();
        let idx = if list[length - 2] == "{{closure}}" { length - 3 } else { length - 2 };

        format!("{}:{}:{}", caller.file(), caller.line(), list[idx])
    }};
}

pub mod configuration;
pub mod routes;

use actix_web::{dev::Server, web, App, HttpServer};
// use routes::misc::{health_check, healthy, load_open};
use sqlx::PgPool;
use std::{io, net, thread, time::Duration};

pub fn run(listener: net::TcpListener, pool: PgPool, mut workers: usize) -> io::Result<Server> {
    let data = web::Data::new(pool);

    let threads = thread::available_parallelism().unwrap().get();
    if workers == 0 || workers > threads {
        workers = threads;
    }

    let server = HttpServer::new(move || {
        println!("~~~ start http server: {}", func!());

        App::new()
            .route("/health", web::get().to(routes::health_check))
            .service(routes::healthy)
            .configure(routes::load_open)
            // Register the connection as part of the application state
            .app_data(data.clone())
    })
    .keep_alive(Duration::from_secs(60))
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
