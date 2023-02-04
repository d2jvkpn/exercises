use super::{
    db_access as db,
    models::Course,
    response::{Data, Error},
    state::AppState,
};
use actix_web::{web, HttpResponse};
use serde_json::json;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let msg = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(json!({"code":0,"msg":msg}))
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, Error> {
    let tutor_id = params.0;
    // let courses = db::get_courses_for_tutor(&app_state.db, tutor_id).await;
    // HttpResponse::Ok().json("success")
    // HttpResponse::Ok().json(courses)

    // let data = db::get_courses_for_tutor(&app_state.db, tutor_id).await?;
    // Ok(ResponseData(data).into())

    db::get_courses_for_tutor(&app_state.db, tutor_id).await.map(|v| Ok(Data(v).into()))?
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let (tutor_id, course_id) = (params.0, params.1);

    // -> HttpResponse
    // let course = db::get_course_details(&app_state.db, tutor_id, course_id).await;
    // HttpResponse::Ok().json(course)

    db::get_course_details(&app_state.db, tutor_id, course_id).await.map(|v| Ok(Data(v).into()))?
}

pub async fn post_new_course(
    app_state: web::Data<AppState>,
    course: web::Json<Course>,
) -> Result<HttpResponse, Error> {
    // -> HttpResponse
    // let new_course = db::post_new_course(&app_state.db, course.into()).await;
    // HttpResponse::Ok().json(new_course)

    db::post_new_course(&app_state.db, course.into()).await.map(|v| Ok(Data(v).into()))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;

    async fn new_app_data() -> web::Data<AppState> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is unset");
        let db = PgPool::connect(&database_url).await.unwrap();
        web::Data::new(AppState::new(db))
    }

    #[actix_rt::test]
    async fn test_get_all_course_success() {
        let app_data = new_app_data().await;
        let tutor_id = web::Path::from((1,));
        let resp = get_courses_for_tutor(app_data, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_course_details() {
        let app_data = new_app_data().await;
        let params = web::Path::from((1, 2));
        let resp = get_course_details(app_data, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_course_success() {
        let app_data = new_app_data().await;
        let course = Course {
            course_id: 0,
            tutor_id: 1,
            course_name: "This is the next course".into(),
            posted_time: None,
        };

        let resp = post_new_course(app_data, web::Json(course)).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
