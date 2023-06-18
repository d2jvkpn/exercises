use crate::{
    node::{Child, Node},
    queue::Queue,
    traversal,
};
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

    pub fn new_with(value: T) -> Self {
        Self { root: Node::new_child(value), size: 1 }
    }

    pub fn push_iter(item: &Child<T>, value: T) {
        let node = match item {
            None => {
                return;
            }
            Some(v) => v,
        };

        let v = node.borrow().value.clone();

        if value == v {
            return;
        } else if value < v {
            if node.borrow().left.is_none() {
                node.borrow_mut().left = Node::new_child(v);
            } else {
                Self::push_iter(&node.borrow().left, value);
            }
        } else {
            Self::push_iter(&node.borrow().right, value);
        }
    }

    pub fn push(&mut self, value: T) -> &mut Self {
        match &self.root {
            None => self.root = Node::new_child(value),
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
        Self::locate_child(&node.borrow().right, value.clone())
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
            return Some(left.clone());
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

            if parent.borrow().left == Some(target.clone()) {
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

    // TODO: bfs, dfs, rebalance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_binary_tree() {
        let mut tree = Tree::new_with(10);
        println!("tree: {:?}", tree);

        tree.push(5).push(1);
        tree.push(12);
        tree.push(4).push(6).push(8);

        assert_eq!(tree.count(), 7);
        assert!(tree.delete(1));
        assert!(!tree.delete(13));
        assert_eq!(tree.count(), 6);
        println!("tree: {:?}", tree);

        println!("tree.bfs: {:?}", tree.bfs());
        assert_eq!(tree.bfs(), vec![10, 5, 12, 4, 6, 8]);
    }
}
