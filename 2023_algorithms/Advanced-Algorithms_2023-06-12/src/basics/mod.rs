pub mod linked_list_general;
pub mod linked_list_unique;
pub mod tree;

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
