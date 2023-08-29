use super::super::enums::TaskStatus;
use super::super::traits::{Create, Edit, Get};
use super::base::Base;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(title: &str) -> Self {
        let base = Base { title: title.to_string(), status: TaskStatus::Pending };
        Pending { super_struct: base }
    }
}

impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}
