pub mod enums;
mod items;
pub mod structs;
pub mod traits;

use enums::TaskStatus;
pub use items::{ToDoItem, ToDoItems};
use structs::{Done, Pending};

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => ItemTypes::Done(Done::new(title)),
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(title)),
    }
}
