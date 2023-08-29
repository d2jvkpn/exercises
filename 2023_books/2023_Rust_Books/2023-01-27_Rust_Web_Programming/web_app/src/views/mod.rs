mod auth;
mod open;
mod to_do;

use actix_web::web::ServiceConfig;

pub fn factory(app: &mut ServiceConfig) {
    auth::factory(app);
    open::factory(app);
    to_do::factory(app);
}
