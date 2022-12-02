use actix_web::web;

mod api;
use api::auth::factory;

pub fn load_auth(app: &mut web::ServiceConfig) {
    // api::auth::factory(app);
    factory(app);
}
