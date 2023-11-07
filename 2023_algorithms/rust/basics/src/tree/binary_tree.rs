use super::node::{Child, Node};
use crate::structs::queue1::Queue;
use std::fmt::Debug;

#[derive(Debug)]
struct Tree<T> {
    root: Child<T>,
    size: usize,
}

impl<T: Clone + Debug + PartialEq + PartialOrd> Tree<T> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    pub fn push_iter(item: &Child<T>, node: Node<T>) {
        let child = match item {
            None => return,
            Some(v) => v,
        };

        let v = child.borrow().value.clone();

        if node.value == v {
        } else if node.value < v {
            if child.borrow().left.is_none() {
                child.borrow_mut().left = node.into();
            } else {
                Self::push_iter(&child.borrow().left, node);
            }
        } else {
            Self::push_iter(&child.borrow().right, node);
        }
    }

    pub fn push(&mut self, node: Node<T>) -> &mut Self {
        if let Some(v) = &self.root {
            // _ = v.borrow_mut().push_binary(node);
            _ = push(&mut v.borrow_mut(), node);
        } else {
            self.root = node.into()
        }

        self.size += 1;
        self
    }

    pub fn find(&self, value: T) -> Child<T> {
        fn find<T: Debug + PartialEq + Clone>(item: &Child<T>, value: T) -> Child<T> {
            let node = if let Some(v) = item { v } else { return None };

            if node.borrow().value == value {
                return Some(node.clone());
            }

            if let Some(v) = find(&node.borrow().left, value.clone()) {
                return Some(v);
            }

            find(&node.borrow().right, value)
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
        None
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
        None
    }

    // find a match node and return parent and target node
    pub fn locate_child(item: &Child<T>, value: T) -> (Child<T>, Child<T>) {
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
        let (parent, target) = Self::locate_child(&node.borrow().left, value.clone());
        if target.is_some() {
            return (parent, target);
        }

        // iter right
        Self::locate_child(&node.borrow().right, value)
    }

    pub fn locate(&self, value: T) -> (Child<T>, Child<T>) {
        Self::locate_child(&self.root, value)
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
            Some(left.clone())
        } else {
            Self::take_min(&Some(left.clone()))
        }
    }

    pub fn delete(&mut self, value: T) -> bool {
        let (parent, target) = self.locate(value);

        // case 1
        let target = match target {
            Some(v) => v,
            None => return false,
        };

        // case 2
        let parent = match parent {
            Some(v) => v,
            None => {
                self.root = None;
                return true;
            }
        };

        // case 3, neither target.left of target.right is none
        let left = target.borrow_mut().left.take();
        let right = target.borrow_mut().right.take();

        if left.is_none() || right.is_none() {
            let successor = if left.is_none() { right } else { left };

            if parent.borrow().left == Some(target) {
                parent.borrow_mut().left = successor;
            } else {
                parent.borrow_mut().right = successor;
            }
            return true;
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
        true
    }

    fn count_iter(item: &Child<T>, value: &mut usize) {
        let node = if let Some(v) = item { v } else { return };
        Self::count_iter(&node.borrow().left, value);
        *value += 1;
        Self::count_iter(&node.borrow().right, value);
    }

    pub fn count(&self) -> usize {
        let mut value = 0;
        Self::count_iter(&self.root, &mut value);
        value
    }

    fn levels_iter(item: &Child<T>) -> usize {
        let node = if let Some(v) = item { v } else { return 0 };
        let left = Self::levels_iter(&node.borrow().left);
        let right = Self::levels_iter(&node.borrow().right);

        if left > right {
            left + 1
        } else {
            right + 1
        }
    }

    pub fn levels(&self) -> usize {
        Self::levels_iter(&self.root)
    }

    pub fn bfs(&self) -> Vec<T> {
        let mut vec = Vec::new();

        let mut queue = match &self.root {
            None => return vec,
            Some(v) => Queue::new_with(v.clone()),
        };

        while let Some(qn) = queue.pop() {
            if let Some(v) = &qn.borrow().value.borrow().left {
                _ = queue.push(v.clone());
            }

            if let Some(v) = &qn.borrow().value.borrow().right {
                _ = queue.push(v.clone());
            }

            vec.push(qn.borrow().value.borrow().value.clone());
        }

        vec
    }
}

pub fn push<T: std::cmp::PartialOrd>(parent: &mut Node<T>, node: Node<T>) {
    if node.value <= parent.value {
        if let Some(v) = parent.left.take() {
            push(&mut v.borrow_mut(), node); // !!! not *v.borrow_mut().push(value)
            parent.left = Some(v); // must return self.left
        } else {
            parent.left = node.into();
        }
    } else if let Some(v) = parent.right.take() {
        push(&mut v.borrow_mut(), node);
        parent.right = Some(v);
    } else {
        parent.right = node.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_binary_tree() {
        let mut tree = Tree::new();
        tree.root = Node::new(10).into();
        println!("tree: {:?}", tree);

        tree.push(Node::new(5)).push(Node::new(1));
        tree.push(Node::new(12));
        tree.push(Node::new(4)).push(Node::new(6)).push(Node::new(8));

        assert_eq!(tree.count(), 7);
        assert!(tree.delete(1));
        assert!(!tree.delete(13));
        assert_eq!(tree.count(), 6);
        println!("tree: {:?}", tree);

        println!("tree.bfs: {:?}", tree.bfs());
        assert_eq!(tree.bfs(), vec![10, 5, 12, 4, 6, 8]);
    }
}
