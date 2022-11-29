use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    // return "impl Responder" is ok too
    HttpResponse::Ok().finish()
}
