use crate::{node::Node, traversal};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug)]
struct BinaryTree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T: Clone + Debug + PartialEq + PartialOrd> BinaryTree<T> {
    fn new() -> Self {
        Self { root: None, size: 0 }
    }

    fn new_with(value: T) -> Self {
        Self { root: Node::new(value).into_child(), size: 1 }
    }

    fn push(&mut self, value: T) -> &mut Self {
        match &self.root {
            None => self.root = Node::new(value).into_child(),
            Some(v) => {
                v.borrow_mut().push(value);
            }
        }

        self.size += 1;
        self
    }

    pub fn inorder(&self) -> Vec<T> {
        return traversal::inorder_v2(&self.root);
    }

    pub fn preorder(&self) -> Vec<T> {
        return traversal::preorder_v1(&self.root);
    }

    pub fn postorder(&self) -> Vec<T> {
        return traversal::postorder_v1(&self.root);
    }

    // TODO: find, delete
}
