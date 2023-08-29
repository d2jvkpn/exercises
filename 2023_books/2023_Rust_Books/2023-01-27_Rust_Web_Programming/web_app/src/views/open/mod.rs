mod login;

use actix_web::{
    web::{get, post, resource, scope, ServiceConfig},
    HttpRequest, Responder,
};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {name}!")
}

pub fn factory(app: &mut ServiceConfig) {
    let svr = scope("/api/v1/open")
        // .wrap(logger)
        .route("/login", post().to(login::login))
        .service(resource("/greet").route(get().to(greet)))
        .service(resource("/greet/{name}").route(get().to(greet)));

    app.service(svr);
}
