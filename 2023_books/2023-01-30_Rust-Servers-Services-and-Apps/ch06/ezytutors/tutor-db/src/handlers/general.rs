use crate::state::AppState;
use actix_web::{web, HttpResponse};
use serde_json::json;

pub async fn divide(params: web::Path<(i32, i32)>) -> HttpResponse {
    let (numerator, denominator) = (params.0, params.1);

    HttpResponse::Ok().json(json!({"code":0, "msg":"ok", "data": numerator/denominator}))
}

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let msg = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(json!({"code":0,"msg":msg}))
}
