use crate::{common::Resp, routes::subscribe};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web::{self, Json, Query, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use chrono::{Local, SecondsFormat};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    name: Option<String>,
}

pub async fn health_check() -> HttpResponse {
    // return "impl Responder" is ok too
    HttpResponse::Ok().finish()
}

#[get("/healthy")]
pub async fn healthy() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Version", "0.1.0"))
        .body("Ok")
}

pub fn load_open(config: &mut ServiceConfig) {
    config.route("/open/subscribe", web::post().to(subscribe));
    config.route("/open/greet/{name}", web::get().to(greet));
    config.route("/open/hello", web::post().to(hello));
    config.route("/open/hello/{platform}", web::post().to(hello));
}

pub fn load_open2(config: &mut ServiceConfig) {
    let router = web::scope("/open")
        .service(web::resource("/subscribe").route(web::post().to(subscribe)))
        .service(web::resource("/greet/{name}").route(web::get().to(greet)))
        .service(web::resource("/hello").route(web::post().to(hello)))
        .service(web::resource("/hello/{platform}").route(web::post().to(hello)));

    config.service(router);
}

pub async fn greet(req: HttpRequest) -> String {
    let name = req.match_info().get("name").unwrap_or("World");
    let name = &name;

    format!("Hello {}!\n", name)
}

async fn hello(
    req: HttpRequest,
    query: Query<HashMap<String, String>>,
    user: Json<User>,
) -> impl Responder {
    let id: i64 = match query.get("id") {
        Some(v) => v.parse().unwrap_or(0),
        None => 0,
    };
    // let id: i64 = req.match_info().query("id").parse::<i64>().unwrap_or(0);

    let now = Local::now();

    println!("==> caller: {}", func!());

    println!(
        "~~~ {} /open/hello: method={}, platform={:?}, version={:?}, id={}",
        now.to_rfc3339_opts(SecondsFormat::Millis, true),
        req.method(),
        req.match_info().get("platform"),
        req.headers().get("X-Version"),
        id,
    );

    // let mut data = HashMap::new();
    // data.insert("now".to_string(), now.format("%FT%T%:z").to_string());
    // # now.format("%Y-%m-%dT%H:%M:%S%:z")
    let data = HashMap::from([("now".to_string(), now.format("%FT%T%:z").to_string())]);
    let mut resp = Resp::new();
    resp.data(data);

    let name = match &user.name {
        Some(v) => &v,
        None => "",
    };

    if name.is_empty() {
        // (resp.code, resp.msg) = (-1, format!("name isn't provided"));
        resp.code(-1).msg("name isn't provided");
        return (Json(resp), StatusCode::BAD_REQUEST);
    } else if name.len() > 32 {
        resp.code(-2).msg("the length of name excceds limit 32");
        return resp.bad_request();
    }

    resp.msg("Hello, {}!");
    resp.ok()
    // (Json(resp), StatusCode::OK)
    // use serde_json::json;
    // Ok(Json(json!({"code": 0, "msg": "welcome"})))
}
