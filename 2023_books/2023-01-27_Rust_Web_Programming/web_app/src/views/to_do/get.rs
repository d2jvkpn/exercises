use actix_web::Responder;

use crate::to_do::ToDoItems;

pub async fn get() -> impl Responder {
    ToDoItems::get_state()
}
