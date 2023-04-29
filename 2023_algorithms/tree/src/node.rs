use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Rc<RefCell<Node<T>>>>,
    pub right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, left: None, right: None }
    }

    pub fn into_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    pub fn into_child(self) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(self)))
    }

    pub fn push(&mut self, value: T) -> &mut Self {
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

        self
    }

    pub fn path(&self, value: T, steps: &mut Vec<bool>) {
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

    pub fn walk(&self, steps: &[bool]) -> Option<T> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_node() {
        let mut node = Node::new(10);
        println!("{:?}", node);

        node.push(5).push(1);
        node.push(12);
        node.push(4).push(6).push(8);

        println!("{:?}", node);
    }
}
