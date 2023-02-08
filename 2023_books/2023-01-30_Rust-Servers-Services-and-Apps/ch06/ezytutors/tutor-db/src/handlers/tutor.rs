use crate::{
    db,
    models::*,
    response::{Data, Error},
    state::AppState,
};
use actix_web::{web, HttpResponse};

pub async fn post_new_tutor(
    app_state: web::Data<AppState>,
    tutor: web::Json<Tutor>,
) -> Result<HttpResponse, Error> {
    db::post_new_tutor(&app_state.db, tutor.into_inner()).await.map(|v| Ok(Data(v).into()))?
}

pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let tutor_id = params.into_inner();

    db::get_tutor_details(&app_state.db, tutor_id).await.map(|v| Ok(Data(v).into()))?
}

pub async fn get_all_tutors(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    db::get_all_tutors(&app_state.db).await.map(|v| Ok(Data(v).into()))?
}

pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    update_tutor: web::Json<UpdateTutor>,
) -> Result<HttpResponse, Error> {
    let tutor_id = params.into_inner();

    db::update_tutor_details(&app_state.db, tutor_id, update_tutor.into_inner())
        .await
        .map(|v| Ok(Data(v).into()))?
}

pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let tutor_id = params.into_inner();
    db::delete_tutor(&app_state.db, tutor_id).await.map(|v| Ok(Data(v).into()))?
}
