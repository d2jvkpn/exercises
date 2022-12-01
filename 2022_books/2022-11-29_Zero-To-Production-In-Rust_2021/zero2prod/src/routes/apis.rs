// use crate::{common::Resp};
use super::common::Resp;
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web::{self, Json, Query},
    HttpRequest, HttpResponse, Responder, Result,
};
use chrono::{Local, SecondsFormat};
use serde::{self, Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

// health check
pub async fn healthz() -> HttpResponse {
    // return "impl Responder" is ok too
    HttpResponse::Ok().finish()
}

#[get("/healthy")]
pub async fn healthy() -> impl Responder {
    //    HttpResponse::Ok()
    //        .content_type(ContentType::plaintext())
    //        .insert_header(("X-Version", "0.1.0"))
    //        .body("Ok")

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .insert_header(("X-Version", "0.1.0"))
        .body(json!({"code":0,"msg":"ok"}).to_string())
}

// extract data from path and query
#[derive(Deserialize)]
pub(super) struct Info {
    user_id: u32,
    friend: String,
}

#[get("/open/info/{user_id}/{friend}")] // <- define path parameters
pub(super) async fn info(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!\n", info.friend, info.user_id))
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Params {
    page_no: Option<u16>,
    page_size: Option<u16>,
}

pub(super) async fn greet(
    req: HttpRequest,
    params: web::Query<Params>,
    request_id: Option<web::ReqData<Uuid>>,
) -> String {
    let name = req.match_info().get("name").unwrap_or("World");
    let name = &name;

    format!("Hello {}, {:?}, request_id: {:?}!\n", name, params, request_id)
}

// query, json body, and middleware-ext-mut
// https://github.com/actix/examples/tree/master/middleware/middleware-ext-mut
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct User {
    name: Option<String>,
}

pub(super) async fn hello(
    req: HttpRequest,
    query: Query<HashMap<String, String>>,
    user: Json<User>,
    request_id: Option<web::ReqData<Uuid>>,
) -> impl Responder {
    let id: i64 = match query.get("id") {
        Some(v) => v.parse().unwrap_or(0),
        None => 0,
    };
    // let id: i64 = req.match_info().query("id").parse::<i64>().unwrap_or(0);

    let now = Local::now();

    println!(
        "~~~ {} {}: method={}, path: {}?{}, platform={:?}, version={:?}, id={}",
        now.to_rfc3339_opts(SecondsFormat::Millis, true),
        func!(),
        req.method(),
        req.path(),
        req.query_string(),
        req.match_info().get("platform"),
        req.headers().get("X-Version"),
        id,
    );

    // let mut data = HashMap::new();
    // data.insert("now".to_string(), now.format("%FT%T%:z").to_string());
    // # now.format("%Y-%m-%dT%H:%M:%S%:z")
    let data = HashMap::from([("now", now.format("%FT%T%:z").to_string())]);

    let request_id: Uuid = match request_id {
        Some(v) => v.into_inner(), // v: web::ReqData<Uuid>
        None => Uuid::new_v4(),
    };
    let mut resp = Resp::new(Some(request_id));
    resp.data(data);

    let name = user.name.as_deref().unwrap_or("");

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
