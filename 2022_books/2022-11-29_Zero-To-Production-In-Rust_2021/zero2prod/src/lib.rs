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

use actix_web::{
    dev::{Server, Service as _},
    web, App, HttpServer,
};
use futures_util::future::FutureExt;
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
            // middlewares .wrap(f1).wrap(f2).wrap(f3), execution order f3() -> f2() -> f1()
            .wrap_fn(|req, srv| {
                println!("--> Hi from start. You requested: {}", req.path());
                srv.call(req).map(|res| {
                    println!("<-- Hi from response");
                    res
                })
            })
            // .wrap(routes::middlewares::SimpleLogger)
            .route("/healthz", web::get().to(routes::healthz))
            .service(routes::healthy)
            .configure(routes::open_scope)
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
    use crate::routes::healthz;

    #[actix_rt::test]
    async fn health_check_succeeds() {
        let response = healthz().await;
        assert!(response.status().is_success())
    }
}
