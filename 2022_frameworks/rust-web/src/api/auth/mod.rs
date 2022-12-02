use actix_web::web;

mod path;
mod user;

use path::Path;

/// This function adds the auth views to the web server.
///
/// # Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// # Returns
/// None
pub fn factory(app: &mut web::ServiceConfig) {
    // define the path struct
    let api = Path::new("/api");
    // define the routes for the app

    app.route(&api.define("/open/login/{platform}"), web::post().to(user::login));
    app.route(&api.define("/auth/logout"), web::post().to(user::logout));
}
