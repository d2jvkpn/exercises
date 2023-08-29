use super::handlers::*;
use super::misc::*;
use actix_web::web::{self, get, post, ServiceConfig};

fn general_routes(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(health_check_handler));
}

fn course_routes(cfg: &mut ServiceConfig) {
    let scope = web::scope("/courses")
        .route("/create", post().to(post_new_course))
        .route("/{tutor_id}", get().to(get_courses_for_tutor))
        .route("/{tutor_id}/{course_id}", get().to(get_course_details));

    let misc = web::scope("/misc").route("/divide/{numerator}/{denominator}", get().to(divide));

    cfg.service(scope).service(misc);
}

pub fn route(cfg: &mut ServiceConfig) {
    general_routes(cfg);
    course_routes(cfg);
}
