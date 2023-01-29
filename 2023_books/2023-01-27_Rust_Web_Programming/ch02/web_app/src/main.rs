mod middlewares;
mod processes;
mod state;
mod to_do;
mod views;

use actix_service::Service;
use actix_web::{middleware, App, HttpServer};

static STATE_FILE: &str = "data/state.json";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8000";
    println!(">>> HTTP Server is listening on {:?}", addr);

    HttpServer::new(|| {
        println!("    http server factory is firing, ThreadId={:?}", std::thread::current().id());

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middlewares::logger::SimpleLogger)
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::factory)
    })
    .bind(addr)?
    .workers(4)
    .run()
    .await
}

pub fn get_state() -> &'static str {
    return STATE_FILE;
}
