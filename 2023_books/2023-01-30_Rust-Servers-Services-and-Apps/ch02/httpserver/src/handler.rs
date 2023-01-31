use http::{HttpRequest, HttpResponse, Resource};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs};

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;

    fn load_file(fp: &str) -> Option<String> {
        let fp = "/".to_owned()
            + &fp
                .split("/")
                .filter(|v| !&[".", "..", ""].contains(v))
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("/");

        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, fp);

        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

pub struct StaticPageHandler;

pub struct PageNotFoundHandler;

pub struct WebServiceHandler;

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(p) = &req.resource;

        let p = match p.as_str() {
            "/" | "/index.html" => {
                return HttpResponse::new("200", None, Self::load_file("index.html"))
            }
            "/health" => return HttpResponse::new("200", None, Self::load_file("health.html")),
            v => v,
        };

        match Self::load_file(p) {
            Some(v) => {
                let mut headers: HashMap<&str, &str> = HashMap::new();

                if p.ends_with(".css") {
                    headers.insert("Content-Type", "text/css");
                } else if p.ends_with(".js") {
                    headers.insert("Content-Type", "text/javascript");
                } else {
                    headers.insert("Content-Type", "text/html");
                }

                HttpResponse::new("200", Some(headers), Some(v))
            }
            None => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}

// Define a load_json() method to load orders.json file from disk
impl WebServiceHandler {
    fn load_json(fp: &str) -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, fp);
        let json_contents = fs::read_to_string(full_path);

        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_contents.unwrap().as_str()).unwrap();

        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(p) = &req.resource;

        match p.as_str() {
            "/api/shipping/orders" => {
                let body = Some(serde_json::to_string(&Self::load_json("orders.json")).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => {
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");

                HttpResponse::new(
                    "404",
                    Some(headers),
                    Some(r#"{"code":-1,"msg":"not found"}"#.to_string()),
                )
            }
        }
    }
}
