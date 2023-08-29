mod logout;

use actix_web::web::{post, scope, ServiceConfig};

pub fn factory(app: &mut ServiceConfig) {
    app.service(scope("/api/v1/auth").route("/logout", post().to(logout::logout)));
}
