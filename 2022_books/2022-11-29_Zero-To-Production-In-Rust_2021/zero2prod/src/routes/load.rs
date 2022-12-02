use super::{apis::*, middlewares::SimpleLogger, subscriptions::subscribe};
use actix_web::web::{get, post, resource, scope, ServiceConfig};

pub fn open_route(config: &mut ServiceConfig) {
    config
        .route("/open/subscribe", post().to(subscribe))
        .route("/open/greet", get().to(greet))
        .route("/open/greet/{name}", get().to(greet))
        .route("/open/hello", post().to(hello))
        .route("/open/hello/{platform}", post().to(hello))
        .service(info)
        .service(healthy);
}

pub fn open_scope(config: &mut ServiceConfig) {
    let logger = SimpleLogger {};

    let scope_service = scope("/open")
        .wrap(logger)
        .service(resource("/subscribe").route(post().to(subscribe)))
        .service(resource("/greet").route(get().to(greet)))
        .service(resource("/greet/{name}").route(get().to(greet)))
        .service(resource("/hello").route(post().to(hello)))
        .service(resource("/hello/{platform}").route(post().to(hello)));

    config.service(info).service(healthy).service(scope_service);
}
