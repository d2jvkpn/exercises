use super::node::{Child, Node};
use crate::structs::queue_linked_list::Queue;
use std::{cmp::PartialOrd, fmt::Debug};

#[derive(Debug)]
pub struct Tree<T> {
    pub root: Child<T>,
    size: usize,
}

impl<T: Clone + Debug + PartialEq + PartialOrd> Tree<T> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    pub fn clear(&mut self) {
        _ = self.root.take();
        self.size = 0;
    }

    pub fn root_value(&self) -> Option<T> {
        let node = self.root.clone()?;
        let ans = Some(node.borrow().data.clone());
        ans
    }

    pub fn push_iter(item: &Child<T>, node: Node<T>) {
        let child = match item {
            None => return,
            Some(v) => v,
        };

        let v = child.borrow().data.clone();

        if node.data == v {
        } else if node.data < v {
            if child.borrow().left.is_none() {
                child.borrow_mut().left = node.into();
            } else {
                Self::push_iter(&child.borrow().left, node);
            }
        } else {
            Self::push_iter(&child.borrow().right, node);
        }
    }

    pub fn push_node(&mut self, node: Node<T>) -> &mut Self {
        if let Some(v) = &self.root {
            // _ = v.borrow_mut().push_binary(node);
            _ = push_recur(&mut v.borrow_mut(), node);
        } else {
            self.root = node.into()
        }

        self.size += 1;
        self
    }

    pub fn push(&mut self, data: T) -> &mut Self {
        self.push_node(Node::new(data))
    }

    pub fn push_slice(&mut self, slice: &[T]) {
        slice.iter().for_each(|v| {
            self.push(v.clone());
        });
    }

    pub fn find(&self, data: T) -> Child<T> {
        fn find<T: Debug + PartialEq + Clone>(item: &Child<T>, data: T) -> Child<T> {
            let node = if let Some(v) = item { v } else { return None };

            if node.borrow().data == data {
                return Some(node.clone());
            }

            if let Some(v) = find(&node.borrow().left, data.clone()) {
                return Some(v);
            }

            find(&node.borrow().right, data)
        }

        find(&self.root, data)
    }

    fn match_left(item: &Child<T>, data: T) -> Child<T> {
        let node = if let Some(v) = item { v.borrow() } else { return None };

        let node = match &node.left {
            Some(v) => v,
            None => return None,
        };

        if node.borrow().data == data {
            return Some(node.clone());
        }
        None
    }

    fn match_right(item: &Child<T>, data: T) -> Child<T> {
        let node = if let Some(v) = item { v.borrow() } else { return None };

        let node = match &node.right {
            Some(v) => v,
            None => return None,
        };

        if node.borrow().data == data {
            return Some(node.clone());
        }
        None
    }

    // find a match node and return parent and target node
    fn locate_recur(item: &Child<T>, data: T) -> (Child<T>, Child<T>) {
        // case 1: root node is None
        let node = if let Some(v) = item { v } else { return (None, None) };

        if node.borrow().data == data {
            // case 2: target is root node
            return (None, Some(node.clone()));
        }

        // case 3: match left
        let target = Self::match_left(&Some(node.clone()), data.clone());
        if target.is_some() {
            return (Some(node.clone()), target);
        }

        // case 4: match right
        let target = Self::match_right(&Some(node.clone()), data.clone());
        if target.is_some() {
            return (Some(node.clone()), target);
        }

        // iter left node
        let (parent, target) = Self::locate_recur(&node.borrow().left, data.clone());
        if target.is_some() {
            return (parent, target);
        }

        // iter right node
        Self::locate_recur(&node.borrow().right, data)
    }

    pub fn locate(&self, data: T) -> (Child<T>, Child<T>) {
        Self::locate_recur(&self.root, data)
    }

    fn take_min(item: &Child<T>) -> (Child<T>, Child<T>) {
        // parent, target
        let node = if let Some(v) = item {
            v
        } else {
            return (None, None); // not found
        };

        let binding = node.borrow();
        let left = match &binding.left {
            None => return (None, Some(node.clone())), // has no parent
            Some(v) => v,
        };

        if left.borrow().left.is_none() {
            node.borrow_mut().left = left.borrow_mut().right.take();
            return (Some(node.clone()), Some(left.clone()));
        }

        Self::take_min(&Some(left.clone()))
    }

    fn take_max(item: &Child<T>) -> (Child<T>, Child<T>) {
        // parent, target
        let node = if let Some(v) = item {
            v
        } else {
            return (None, None); // not found
        };

        let binding = node.borrow();
        let right = match &binding.right {
            None => return (None, Some(node.clone())), // has no parent
            Some(v) => v,
        };

        if right.borrow().right.is_none() {
            node.borrow_mut().right = right.borrow_mut().left.take();
            return (Some(node.clone()), Some(right.clone()));
        }

        Self::take_min(&Some(right.clone()))
    }

    pub fn remove(&mut self, data: T) -> bool {
        let (parent, target) = self.locate(data);
        let pair: (Child<T>, Child<T>);
        let mut successor: Child<T>;

        let target = if let Some(v) = target {
            self.size -= 1;
            v
        } else {
            dbg!("--> case 1: root is None");
            return false;
        };

        let left = target.borrow_mut().left.take();
        let right = target.borrow_mut().right.take();

        if parent.is_none() {
            dbg!("--> case 2: root is target");
            pair = Self::take_min(&right);
            successor = pair.1;

            match &successor {
                None => successor = left,
                Some(v) => v.borrow_mut().left = left,
            }
        } else if left.is_none() && right.is_none() {
            dbg!("--> case 3: target is a leaf node");
            successor = None;
        } else if left.is_none() || right.is_none() {
            dbg!("--> case 4: target only has one child");
            successor = if right.is_none() { left } else { right };
        } else {
            dbg!("--> case 5: both targte.left and target.right are some");
            pair = Self::take_min(&right);
            successor = pair.1;
            let node = successor.clone().unwrap();
            node.borrow_mut().left = left;
            node.borrow_mut().right = right;
        }

        match parent {
            None => {
                _ = self.root.take();
                self.root = successor;
            }
            Some(v) => {
                if v.borrow().left == Some(target) {
                    v.borrow_mut().left = successor;
                } else {
                    v.borrow_mut().right = successor;
                }
            }
        }

        true
    }

    pub fn count(&self) -> usize {
        match &self.root {
            None => 0,
            Some(v) => v.borrow().count(),
        }
    }

    pub fn height(&self) -> usize {
        match &self.root {
            None => 0,
            Some(v) => v.borrow().height(),
        }
    }

    pub fn bfs_vector(&self) -> Vec<T> {
        let mut ans = Vec::new();

        let mut queue = match &self.root {
            None => return ans,
            Some(v) => Queue::new_with(v.clone()),
        };

        while let Some(node) = queue.pop() {
            if let Some(v) = &node.borrow().left {
                _ = queue.push(v.clone());
            }

            if let Some(v) = &node.borrow().right {
                _ = queue.push(v.clone());
            }

            ans.push(node.borrow().data.clone());
        }

        ans
    }

    pub fn bfs(&self, call: fn(&T)) {
        let mut queue = match &self.root {
            None => return,
            Some(v) => Queue::new_with(v.clone()),
        };

        while let Some(node) = queue.pop() {
            if let Some(v) = &node.borrow().left {
                _ = queue.push(v.clone());
            }

            if let Some(v) = &node.borrow().right {
                _ = queue.push(v.clone());
            }

            call(&node.clone().borrow().data);
        }
    }

    //
    fn inorder_recur(node: &Child<T>, call: fn(&T)) {
        let node = if let Some(v) = node { v } else { return };

        Self::inorder_recur(&node.borrow().left, call);
        call(&node.borrow().data);
        Self::inorder_recur(&node.borrow().right, call);
    }

    pub fn inorder(&self, call: fn(&T)) {
        Self::inorder_recur(&self.root, call)
    }

    //
    fn get_range_recur(item: &Child<T>, low: &T, high: &T, vec: &mut Vec<T>) {
        let node = if let Some(v) = item { v.borrow() } else { return };

        Self::get_range_recur(&node.left, low, high, vec);

        let data = &node.data;
        if data >= low && data <= high {
            vec.push(data.clone());
        }

        Self::get_range_recur(&node.right, low, high, vec);
    }

    pub fn get_range(&self, low: T, high: T) -> Vec<T> {
        let mut ans = Vec::new();

        Self::get_range_recur(&self.root, &low, &high, &mut ans);

        ans
    }
}

pub fn push_recur<T: PartialOrd>(parent: &mut Node<T>, node: Node<T>) {
    if node.data <= parent.data {
        if let Some(v) = parent.left.take() {
            push_recur(&mut v.borrow_mut(), node); // !!! not *v.borrow_mut().push(data)
            parent.left = Some(v); // must return self.left
        } else {
            parent.left = node.into();
        }
    } else if let Some(v) = parent.right.take() {
        push_recur(&mut v.borrow_mut(), node);
        parent.right = Some(v);
    } else {
        parent.right = node.into();
    }
}
