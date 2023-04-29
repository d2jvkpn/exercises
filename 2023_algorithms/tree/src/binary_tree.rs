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

    // find a match node and return parent and target node
    pub fn local_child(item: &Child<T>, value: T) -> (Child<T>, Child<T>) {
        // case 1
        let node = if let Some(v) = item { v } else { return (None, None) };

        if node.borrow().value == value {
            // case 2
            return (None, Some(node.clone()));
        }

        // case 3
        let target = Self::match_left(&Some(node.clone()), value.clone());
        if target.is_some() {
            return (Some(node.clone()), target);
        }

        // case 4
        let target = Self::match_right(&Some(node.clone()), value.clone());
        if target.is_some() {
            return (Some(node.clone()), target);
        }

        // iter left
        let (parent, target) = Self::local_child(&node.borrow().left, value.clone());
        if target.is_some() {
            return (parent, target);
        }

        // iter right
        Self::local_child(&node.borrow().right, value.clone())
    }

    pub fn local(&self, value: T) -> (Child<T>, Child<T>) {
        Self::local_child(&self.root, value)
    }

    fn take_min(item: &Child<T>) -> Child<T> {
        let node = if let Some(v) = item { v } else { return None };

        let binding = node.borrow();
        let left = match &binding.left {
            None => return None,
            Some(v) => v,
        };

        if left.borrow().left.is_none() {
            node.borrow_mut().left = left.borrow_mut().right.take();
            return Some(left.clone());
        } else {
            Self::take_min(&Some(left.clone()))
        }
    }

    pub fn delete(&mut self, value: T) {
        let (parent, target) = self.local(value);

        // case 1
        let target = match target {
            Some(v) => v,
            None => return,
        };

        // case 2
        let parent = match parent {
            Some(v) => v,
            None => {
                self.root = None;
                return;
            }
        };

        // case 3, neither target.left of target.right is none
        let left = target.borrow_mut().left.take();
        let right = target.borrow_mut().right.take();

        if left.is_none() || right.is_none() {
            let successor = if left.is_none() { right } else { left };

            if parent.borrow().left == Some(target.clone()) {
                parent.borrow_mut().left = successor;
            } else {
                parent.borrow_mut().right = successor;
            }
            return;
        }

        // case 4, both targte.left and target.right are some
        let replace = match Self::take_min(&right) {
            None => right, // the right has no left
            Some(v) => {
                v.borrow_mut().right = right;
                Some(v)
            }
        };

        replace.clone().unwrap().borrow_mut().left = left;
        parent.borrow_mut().right = replace;
    }

    // TODO: rebalance
}
