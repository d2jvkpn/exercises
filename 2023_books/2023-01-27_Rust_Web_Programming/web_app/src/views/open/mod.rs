mod login;

use actix_web::web::{post, scope, ServiceConfig};

pub fn factory(app: &mut ServiceConfig) {
    app.service(scope("/api/v1/open").route("/login", post().to(login::login)));
}
