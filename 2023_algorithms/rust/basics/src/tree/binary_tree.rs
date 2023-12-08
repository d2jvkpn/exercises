use super::node::{Child, Node, Side};
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

    pub fn push2node_v1(parent: &mut Node<T>, node: Node<T>) -> bool {
        let ans: bool;

        if node.data == parent.data {
            ans = false;
        } else if node.data < parent.data {
            if let Some(v) = parent.left.take() {
                // !!! not *v.borrow_mut().push2node_v1(data)
                ans = Self::push2node_v1(&mut v.borrow_mut(), node);
                parent.left = Some(v); // must return self.left
            } else {
                parent.left = node.into();
                ans = true;
            }
        } else {
            if let Some(v) = parent.right.take() {
                ans = Self::push2node_v1(&mut v.borrow_mut(), node);
                parent.right = Some(v);
            } else {
                parent.right = node.into();
                ans = true;
            }
        }

        ans
    }

    pub fn push2node_v2(parent: &mut Node<T>, node: Node<T>) -> bool {
        let ans: bool;

        let side = if node.data == parent.data {
            return false;
        } else if node.data < parent.data {
            Side::Left
        } else {
            Side::Right
        };

        if let Some(v) = parent.take_child(side) {
            // !!! not *v.borrow_mut().push2node_v1(data)
            ans = Self::push2node_v2(&mut v.borrow_mut(), node);
            parent.set_child(side, Some(v)); // must return self.left
        } else {
            parent.set_child(side, node.into());
            ans = true;
        }

        ans
    }

    pub fn push2child(item: &Child<T>, node: Node<T>) -> bool {
        let child = match item {
            None => return false,
            Some(v) => v,
        };

        let v = child.borrow().data.clone();

        /*
        if node.data < v {
            if child.borrow().left.is_none() {
                child.borrow_mut().left = node.into();
                true
            } else {
                Self::push2child(&child.borrow().left, node)
            }
        } else {
            if child.borrow().right.is_none() {
                child.borrow_mut().right = node.into();
                true
            } else {
                Self::push2child(&child.borrow().right, node)
            }
        }
        */

        let side = if node.data == v {
            return false;
        } else if node.data < v {
            Side::Left
        } else {
            Side::Right
        };

        if child.borrow().child(side).is_none() {
            child.borrow_mut().set_child(side, node.into());
            true
        } else {
            Self::push2child(child.borrow().child(side), node)
        }
    }

    pub fn push_node(&mut self, node: Node<T>) -> &mut Self {
        let pushed: bool;

        if let Some(v) = &self.root {
            // _ = v.borrow_mut().push_binary(node);
            // pushed = Self::push2node_v1(&mut v.borrow_mut(), node);
            pushed = Self::push2child(&Some(v.clone()), node);
        } else {
            self.root = node.into();
            pushed = true;
        }

        if pushed {
            self.size += 1;
        }
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

    pub fn child_eq(item: &Child<T>, data: &T) -> bool {
        match item {
            Some(v) => &v.borrow().data == data,
            None => false,
        }
    }

    /* Find a match node and return parent, side and target node
    - None, None, None
    - None, None, Some
    - Some, Some, Some
    **/
    fn locate_recur(item: &Child<T>, data: &T) -> (Child<T>, Option<Side>, Child<T>) {
        // case 1: root node is None
        let node = if let Some(v) = item { v } else { return (None, None, None) };

        if &node.borrow().data == data {
            // case 2: target is root node
            return (None, None, Some(node.clone()));
        }

        // case 3: match left or right
        if Self::child_eq(&node.borrow().left, data) {
            return (Some(node.clone()), Some(Side::Left), node.borrow().left.clone());
        }

        if Self::child_eq(&node.borrow().right, data) {
            return (Some(node.clone()), Some(Side::Right), node.borrow().right.clone());
        }

        // case 4: iter left node and right node
        let ans = Self::locate_recur(&node.borrow().left, data);
        if ans.0.is_some() {
            ans
        } else {
            Self::locate_recur(&node.borrow().right, data)
        }
    }

    pub fn locate(&self, data: &T) -> (Child<T>, Option<Side>, Child<T>) {
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

    pub fn remove(&mut self, data: &T) -> bool {
        let (parent, side, target) = self.locate(data);
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
            Some(v) => v.borrow_mut().set_child(side.unwrap(), successor),
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
