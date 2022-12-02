use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    email: String,
    password: String,
}

pub async fn login(
    req: HttpRequest,
    query: web::Query<HashMap<String, String>>,
    user: web::Json<User>,
) -> Result<impl Responder> {
    let language: &str = match req.headers().get("X-Language") {
        Some(v) => v.to_str().unwrap_or(""),
        None => "",
    };

    let platform = req.match_info().get("platform").unwrap_or("");
    let timestamp = query.get("timestamp");

    println!(
        "~~ {:?}, language={}, platform={}, timestamp={:?}",
        user, language, platform, timestamp
    );
    Ok(HttpResponse::Ok().json(user))
    // Ok(web::Json(json!({"code": 0, "msg": "welcome"}))) // serde_json
}

pub async fn logout() -> String {
    format!("Logout view\n")
}
