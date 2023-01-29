use actix_web::Responder;

use crate::to_do::ToDoItems;

pub async fn get() -> impl Responder {
    return ToDoItems::get_state();
}
