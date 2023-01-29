pub mod enums;
pub mod structs;
mod to_do_item;
mod to_do_items;
pub mod traits;

use enums::TaskStatus;
use structs::{Done, Pending};
pub use to_do_item::ToDoItem;
pub use to_do_items::ToDoItems;

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
