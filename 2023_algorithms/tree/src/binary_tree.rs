use crate::{
    node::{Child, Node},
    traversal,
};
use std::fmt::Debug;

#[derive(Debug)]
struct BinaryTree<T> {
    root: Child<T>,
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

    pub fn find(&self, value: T) -> Child<T> {
        fn find<T: Debug + PartialEq + Clone>(item: &Child<T>, value: T) -> Child<T> {
            let node = if let Some(v) = item { v } else { return None };

            if node.borrow().value == value.clone() {
                return Some(node.clone());
            }

            if let Some(v) = find(&node.borrow().left, value.clone()) {
                return Some(v);
            }

            find(&node.borrow().right, value.clone())
        }

        find(&self.root, value)
    }

    fn match_left(item: &Child<T>, value: T) -> Child<T> {
        let node = if let Some(v) = item { v } else { return None };

        let child = node.borrow();
        let child = match &child.left {
            Some(v) => v,
            None => return None,
        };

        if child.borrow().value == value {
            return Some(child.clone());
        }
        return None;
    }

    fn match_right(item: &Child<T>, value: T) -> Child<T> {
        let node = if let Some(v) = item { v } else { return None };

        let child = node.borrow();
        let child = match &child.right {
            Some(v) => v,
            None => return None,
        };

        if child.borrow().value == value {
            return Some(child.clone());
        }
        return None;
    }

    // TODO: delete
}
