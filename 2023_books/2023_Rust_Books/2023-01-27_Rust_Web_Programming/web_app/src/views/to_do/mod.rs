mod create;
mod edit;
mod get;

use actix_web::web::{get, post, scope, ServiceConfig};

pub fn factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1/item")
            .route("/create/{title}", post().to(create::create))
            .route("/get", get().to(get::get))
            .route("/edit", post().to(edit::edit)),
    );
}
