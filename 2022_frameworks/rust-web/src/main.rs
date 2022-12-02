use actix_web::{middleware, web, App, HttpServer};
use futures::future;
use structopt::StructOpt;

mod demo;

use rust_web::load_auth;

use std::{io, net::SocketAddr, path::PathBuf};

#[allow(dead_code)]
#[derive(Debug, StructOpt)]
#[structopt(name = "rust-web", about = "An example of rust web app")]
struct Opt {
    #[structopt(long)]
    release: bool,

    #[structopt(parse(from_os_str))]
    config: Option<PathBuf>,

    #[structopt(long = "addr", default_value = "0.0.0.0:8080")]
    addr: SocketAddr,

    #[structopt(long = "demo", default_value = "0.0.0.0:8081")]
    demo: SocketAddr,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    // println!("~~~ {:?}", opt);

    if !opt.release {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    }

    let http_addr = opt.addr.to_string();
    println!(">>> Http server is listening on {}", &http_addr);

    let http_server = HttpServer::new(move || {
        let app = App::new();

        let welcome = web::post().to(demo::welcome);
        let entry = web::resource("/welcome").route(welcome);

        let scope = web::scope("/api");
        let router = scope.service(entry);

        // return app.configure(load_auth).service(router).service(v1::hello);
        app.wrap(middleware::Logger::default())
            .configure(load_auth)
            .service(router)
            .service(demo::healthy)
    })
    .bind(&http_addr)?;

    let demo_addr = opt.demo.to_string();
    println!(">>> Demo server is listening on {}", demo_addr);

    let demo_server = HttpServer::new(move || {
        let app: App<_> = App::new();

        let scope = web::scope("/demo");

        let router = scope
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.1")))
            .wrap(middleware::Logger::default())
            .service(demo::healthy)
            .route("/welcome", web::post().to(demo::welcome))
            .route("/index", web::get().to(demo::index))
            .route("/hello/{language}", web::post().to(demo::hello));

        return app.service(router);
    })
    .bind(demo_addr)?;

    // join both server futures and run them
    future::try_join(http_server.workers(4).run(), demo_server.workers(1).run()).await?;
    // future::join(http_server.workers(4).run(), demo_server.workers(1).run()).await;
    Ok(())
}
