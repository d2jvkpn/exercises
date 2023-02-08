use super::{course::*, general::*, tutor::*};
use actix_web::web::{self, delete, get, post, ServiceConfig};

fn general_routes(cfg: &mut ServiceConfig) {
    let misc = web::scope("/misc").route("/divide/{numerator}/{denominator}", get().to(divide));

    cfg.route("/health", get().to(health_check_handler)).service(misc);
}

fn course_routes(cfg: &mut ServiceConfig) {
    let scope = web::scope("/courses")
        .route("/create", post().to(post_new_course))
        .route("/{tutor_id}", get().to(get_courses_for_tutor))
        .route("/{tutor_id}/{course_id}", get().to(get_course_details))
        .route("/{tutor_id}/{course_id}", post().to(update_course_details))
        .route("/{tutor_id}/{course_id}", delete().to(delete_course));

    cfg.service(scope);
}

fn tutor_routes(cfg: &mut ServiceConfig) {
    let scope = web::scope("/tutors")
        .route("/create", post().to(post_new_tutor))
        .route("/{tutor_id}", get().to(get_tutor_details))
        .route("/all", get().to(get_all_tutors))
        .route("/{tutor_id}", post().to(update_tutor_details))
        .route("/{tutor_id}", delete().to(delete_tutor));

    cfg.service(scope);
}

pub fn route(cfg: &mut ServiceConfig) {
    general_routes(cfg);
    course_routes(cfg);
    tutor_routes(cfg);
}
