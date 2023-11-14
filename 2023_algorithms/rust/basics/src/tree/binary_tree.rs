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

    pub fn clear(&mut self) {
        _ = self.root.take();
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

    pub fn remove(&mut self, data: T) -> bool {
        let (parent, target) = self.locate(data);
        let pair: (Child<T>, Child<T>);
        let mut successor: Child<T>;

        // case 1: root is None
        dbg!("--> case 1");
        let target = if let Some(v) = target { v } else { return false };

        let left = target.borrow_mut().left.take();
        let right = target.borrow_mut().right.take();

        if parent.is_none() {
            // case 2: root is target
            dbg!("--> case 2");
            pair = Self::take_min(&right);
            successor = pair.1;

            match &successor {
                None => successor = left,
                Some(v) => v.borrow_mut().left = left,
            }
        } else if left.is_none() && right.is_none() {
            // case 3: target is a leaf node
            dbg!("--> case 3");
            successor = None;
        } else if left.is_none() || right.is_none() {
            // case 4: target only has one child
            dbg!("--> case 4");
            successor = if right.is_none() { left } else { right };
        } else {
            // case 5, both targte.left and target.right are some
            dbg!("--> case 5");
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

    pub fn bfs(&self) -> Vec<T> {
        let mut vec = Vec::new();

        let mut queue = match &self.root {
            None => return vec,
            Some(v) => Queue::new_with(v.clone()),
        };

        while let Some(qn) = queue.pop() {
            if let Some(v) = &qn.borrow().data.borrow().left {
                _ = queue.push(v.clone());
            }

            if let Some(v) = &qn.borrow().data.borrow().right {
                _ = queue.push(v.clone());
            }

            vec.push(qn.borrow().data.borrow().data.clone());
        }

        vec
    }
}

pub fn push_recur<T: std::cmp::PartialOrd>(parent: &mut Node<T>, node: Node<T>) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::traversal::*;

    #[test]
    fn t1_binary_tree() {
        let mut tree = Tree::new();
        tree.root = Node::new(10).into();
        println!("tree: {:?}", tree);

        tree.push(5).push(1);
        tree.push(12);
        tree.push(4).push(6).push(8);

        assert_eq!(tree.count(), 7);
        assert!(tree.remove(1));
        assert!(!tree.remove(13));
        assert_eq!(tree.count(), 6);
        println!("tree: {:?}", tree);

        println!("tree.bfs: {:?}", tree.bfs());
        assert_eq!(tree.bfs(), vec![10, 5, 12, 4, 6, 8]);
    }

    #[test]
    fn t2_binary_tree() {
        let mut tree: Tree<usize>;
        let mut slice: &[usize];
        tree = Tree::new();

        // tree.push(8).push(3).push(10).push(1).push(6).push(14).push(4).push(7).push(13).push(19);
        tree.clear();
        slice = &[8, 3, 10, 1, 6, 14, 4, 7, 13, 19];
        tree.push_slice(slice);
        println!("==> bfs: {:?}", tree.bfs());

        tree.remove(8);
        println!("==> 1. {:?}, remove {}, inorder: {:?}", slice, 8, inorder_recur_a(&tree.root));

        tree.clear();
        slice = &[8, 10, 14];
        tree.push_slice(slice);
        tree.remove(8);
        println!("==> 2. {:?}, remove {}, inorder: {:?}", slice, 8, inorder_recur_a(&tree.root));

        tree.clear();
        slice = &[8, 10, 14];
        tree.push_slice(slice);
        tree.remove(10);
        println!("==> 3. {:?}, remove {}, inorder: {:?}", slice, 10, inorder_recur_a(&tree.root));

        tree.clear();
        slice = &[8];
        tree.push_slice(slice);
        tree.remove(8);
        println!("==> 4. {:?}, remove {}, inorder: {:?}", slice, 8, inorder_recur_a(&tree.root));

        tree.clear();
        slice = &[8, 3];
        tree.push_slice(slice);
        tree.remove(8);
        println!("==> 5. {:?}, remove {}, inorder: {:?}", slice, 8, inorder_recur_a(&tree.root));
    }
}
