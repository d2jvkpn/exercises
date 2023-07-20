#![allow(dead_code)]

mod math;
pub mod searching;
pub mod sorting;
pub mod structs;
pub mod tree;
pub mod utils;

// mod alias
pub mod unique {
    #[doc(inline)]
    pub use super::structs::linked_list_unique::*;
}

// export a sub-module to it's parent
// pub use linked_list_unique::*;

// mod alias
pub mod general {
    #[doc(inline)]
    pub use super::structs::linked_list_general::*;
}
