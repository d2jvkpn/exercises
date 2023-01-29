mod login;
mod logout;

use actix_web::web::{get, scope, ServiceConfig};

pub fn factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
            .route("/open/login", get().to(login::login))
            .route("/auth/logout", get().to(logout::logout)),
    );
}
