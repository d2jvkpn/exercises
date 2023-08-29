use actix_web::{web, HttpResponse};
use serde_json::json;

pub async fn divide(params: web::Path<(i32, i32)>) -> HttpResponse {
    let (numerator, denominator) = (params.0, params.1);

    HttpResponse::Ok().json(json!({"code":0, "msg":"ok", "data": numerator/denominator}))
}
