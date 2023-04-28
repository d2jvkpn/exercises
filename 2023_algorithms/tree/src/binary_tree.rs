use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Rc<RefCell<Node<T>>>>,
    pub right: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
struct BinaryTree<T> {
    root: Node<T>,
    size: usize,
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Node<T> {
    fn new(value: T) -> Self {
        Self { value, left: None, right: None }
    }

    fn into_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    fn push(&mut self, value: T) {
        if value <= self.value {
            if let Some(node) = self.left.take() {
                // println!("<== walk left ({:?}, {:?})", self.value, node.borrow().value);
                (*node).borrow_mut().push(value); // !!! not *node.borrow_mut().push(value)
                self.left = Some(node); // must return self.left
            } else {
                // println!("<++ new left {:?}.left = {:?}\n", self.value, value);
                self.left = Some(Node::new(value).into_rc());
                // println!("{} {:?}", self.value, self.left);
            }
        } else {
            if let Some(node) = self.right.take() {
                // println!("==> walk right ({:?}, {:?})", self.value, node.borrow().value);
                (*node).borrow_mut().push(value);
                self.right = Some(node); // must return to self.right
            } else {
                // println!("++> push right {:?}.right = {:?}\n", self.value, value);
                self.right = Some(Node::new(value).into_rc());
            }
        }
    }

    fn path(&self, value: T, steps: &mut Vec<bool>) {
        if self.value == value {
            return;
        }

        if value < self.value {
            if let Some(node) = &self.left {
                steps.push(false);
                node.borrow().path(value, steps);
            } else {
                *steps = vec![];
            }
        } else {
            if let Some(node) = &self.right {
                steps.push(true);
                node.borrow().path(value, steps);
            } else {
                *steps = vec![];
            }
        }
    }

    fn walk(&self, steps: &[bool]) -> Option<T> {
        if steps.len() == 0 {
            return Some(self.value.clone());
        }

        if !steps[0] {
            match &self.left {
                Some(n) => n.borrow().walk(&steps[1..]),
                None => return None,
            }
        } else {
            match &self.right {
                Some(n) => n.borrow().walk(&steps[1..]),
                None => return None,
            }
        }
        // use a for loop instead??
    }

    // TODO: push, find, delete
}

impl<T: Clone + Debug + PartialEq + PartialOrd> BinaryTree<T> {
    fn new(value: T) -> Self {
        Self { root: Node::new(value), size: 1 }
    }
    // left.borrow_mut().push(value);
    fn push(&mut self, value: T) -> &mut Self {
        self.root.push(value);
        self.size += 1;
        self
    }

    fn path(&self, value: T) -> Option<Vec<bool>> {
        let mut steps = Vec::with_capacity(10);
        steps.push(false); // root node

        self.root.path(value, &mut steps);

        if steps.len() == 0 {
            None
        } else {
            Some(steps[1..].to_vec())
        }
    }

    fn walk(&self, steps: &Vec<bool>) -> Option<T> {
        self.root.walk(&steps[0..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_binary_tree() {
        let mut bt = BinaryTree::new(10);
        println!("{:?}", bt);

        bt.push(5).push(1);
        bt.push(12);
        bt.push(4).push(6).push(8);

        println!("{:?}", bt.root);

        println!("path\t{}\t{:?}", 10, bt.path(10)); // Some([])
        println!("path\t{}\t{:?}", 1, bt.path(1)); // Some([false, false])
        println!("path\t{}\t{:?}", 100, bt.path(100)); // None

        let v1 = bt.path(8).unwrap();
        println!("path\t{}\t{:?}", 8, v1); // Some([false, true, true])
        println!("walk\t{:?}\t{:?}", v1, bt.walk(&v1)); // Some(8)
    }
}
