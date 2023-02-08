use crate::{
    db,
    models::*,
    response::{Data, Error, OK_JSON_STR},
    state::AppState,
};
use actix_web::{web, HttpResponse};

pub async fn post_new_course(
    app_state: web::Data<AppState>,
    course: web::Json<Course>,
) -> Result<HttpResponse, Error> {
    db::post_new_course(&app_state.db, course.into_inner())
        .await
        .map(|_| Ok(Data(OK_JSON_STR).into()))?
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let tutor_id = params.into_inner();

    db::get_courses_for_tutor(&app_state.db, tutor_id).await.map(|v| Ok(Data(v).into()))?
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let (tutor_id, course_id) = params.into_inner(); // (params.0, params.1);

    db::get_course_details(&app_state.db, tutor_id, course_id).await.map(|v| Ok(Data(v).into()))?
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
    update_course: web::Json<UpdateCourse>,
) -> Result<HttpResponse, Error> {
    let (tutor_id, course_id) = params.into_inner(); // (params.0, params.1);

    db::update_course_details(&app_state.db, tutor_id, course_id, update_course.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let (tutor_id, course_id) = params.into_inner();
    db::delete_course(&app_state.db, tutor_id, course_id).await.map(|v| Ok(Data(v).into()))?
}
