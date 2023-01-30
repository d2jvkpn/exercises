use actix_web::{web, HttpResponse};
use serde_json::{value::Value, Map};

use crate::middlewares::jwt::JwToken;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::{self, TaskStatus, ToDoItem, ToDoItems};

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
    println!(">>> Here is the message in the token: {}", token.message);
    let state: Map<String, Value> = read_file();
    let title = &to_do_item.title;

    let status: TaskStatus = match &state.get(title) {
        Some(result) => TaskStatus::from_string(result.as_str().unwrap().to_string()),
        None => return HttpResponse::NotFound().json(format!("{title} not in state")),
    };

    let existing_item = to_do::factory(title, status.clone());

    if status.as_str() == to_do_item.status.as_str() {
        return HttpResponse::Ok().json(ToDoItems::get_state());
    }

    process_input(existing_item, "edit".to_owned(), &state);
    HttpResponse::Ok().json(ToDoItems::get_state())
}
