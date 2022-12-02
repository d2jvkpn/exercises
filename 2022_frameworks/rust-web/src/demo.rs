use actix_web::{
    error::ErrorBadRequest, get, http::header, web, HttpRequest, HttpResponse, Responder, Result,
};

use chrono::{Local, SecondsFormat};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct User {
    name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Resp {
    code: i16,
    msg: String,
    data: HashMap<String, String>,
}

#[get("/healthy")]
pub async fn healthy() -> impl Responder {
    HttpResponse::Ok().body("")
}

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(header::ContentType::plaintext())
        .insert_header(("X-Hdr", "sample"))
        .body("index-data\n")
}

pub async fn welcome(mut user: web::Json<User>) -> String {
    if user.name.is_none() {
        user.name = Some("Jone Doe".to_string());
    }

    format!("Welcome {:?}!\n", user.name)
}

pub async fn hello(
    req: HttpRequest,
    query: web::Query<HashMap<String, String>>,
    user: web::Json<User>,
) -> Result<impl Responder> {
    let id: i64 = match query.get("id") {
        Some(v) => v.parse().unwrap_or(0),
        None => 0,
    };

    let now = Local::now().to_rfc3339_opts(SecondsFormat::Millis, true);

    println!(
        "~~~ {} /../demo_server::hello method={}, language={}, platform={:?}, version: {:?}, id={}",
        now,
        req.method(),
        req.match_info().get("language").unwrap_or(""),
        req.headers().get("X-Platform"),
        req.headers().get("X-Version"),
        id,
    );

    let name = match &user.name {
        Some(v) => &v[..],
        None => "Jone Doe",
    };

    if name.len() > 32 {
        return Err(ErrorBadRequest("name is too long"));
    }

    let data = HashMap::from([("date".to_string(), "2022-10-31".to_string())]);

    Ok(web::Json(Resp { code: 0, msg: format!("Hello, {}!", name), data }))
    // use serde_json::json;
    // Ok(web::Json(json!({"code": 0, "msg": "welcome"})))
}
