use actix_web::HttpRequest;
use serde_json::{value::Value, Map};

use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::{self, TaskStatus};

pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file(); // step 1

    let title: String = req.match_info().get("title").unwrap().to_string(); // step 2

    let item = to_do::factory(title.as_str(), TaskStatus::Pending); // step 3

    process_input(item, "create".to_string(), &state); // step 4

    format!("{title} created") // step 5
}
