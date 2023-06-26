#![allow(dead_code)]

mod math;
mod searching;
mod sorting;
pub mod utils;

pub mod binary_tree;
pub mod doubly_linked_list;
pub mod linked_list_box;
pub mod linked_list_general;
pub mod linked_list_unique;
pub mod queue;
mod stack;
pub mod tree;
pub mod tree_node;
pub mod tree_traversal;

// mod alias
pub mod unique {
    #[doc(inline)]
    pub use super::linked_list_unique::*;
}

// export a sub-module to it's parent
// pub use linked_list_unique::*;

// mod alias
pub mod general {
    #[doc(inline)]
    pub use super::linked_list_general::*;
}
