use serde::Serialize;

use super::super::enums::TaskStatus;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}

impl Base {
    pub fn status_str(&self) -> &'static str {
        self.status.as_str()
    }
}
