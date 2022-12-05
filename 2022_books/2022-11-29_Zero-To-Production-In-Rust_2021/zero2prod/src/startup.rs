use crate::{
    configuration::Settings,
    routes::{
        self,
        handlers::{render_404, render_500},
    },
};
use actix_web::{
    dev::{Server, Service},
    http::StatusCode,
    /* middleware::Logger, */
    middleware::ErrorHandlers,
    web::{self, Data},
    App, HttpServer,
};
use futures_util::future::FutureExt;
use sqlx::PgPool;
use std::{io, net, thread, time::Duration};

// io::Result<Server>
pub fn run(listener: net::TcpListener, pool: PgPool, mut config: Settings) -> io::Result<Server> {
    let data = Data::new(pool);

    let threads = thread::available_parallelism().unwrap().get();
    if config.threads == 0 || config.threads > threads {
        config.threads = threads;
    }

    let server = HttpServer::new(move || {
        // println!("~~~ start http server: {}", func!());
        App::new()
            // Register the connection as part of the application state
            .app_data(data.clone())
            // middlewares .wrap(f1).wrap(f2).wrap(f3), execution order f3() -> f2() -> f1()
            // .wrap(routes::middlewares::SimpleLogger)
            // .wrap(Logger::default())
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, render_404))
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, render_500))
            .wrap_fn(|req, srv| {
                println!("--> Hi from request start, method={}, path={}", req.method(), req.path());
                srv.call(req).map(|res| {
                    let status = res.as_ref().map_or(0, |v| v.status().as_u16());
                    println!("<-- Hi from response: status={}", status);
                    res
                })
            })
            .route("/healthz", web::get().to(routes::healthz))
            .configure(routes::open_scope)
    })
    .keep_alive(Duration::from_secs(config.keep_alive))
    .listen(listener)?
    .workers(config.threads)
    .run();

    Ok(server)
}
