use std::{cell::RefCell, cmp::max, fmt::Debug, rc::Rc};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node<T> {
    pub value: T,
    pub left: Child<T>,
    pub right: Child<T>,
}

pub type Child<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> From<Node<T>> for Child<T> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, left: None, right: None }
    }

    pub fn triangle(value: T, left: T, right: T) -> Self {
        Self { value, left: Self::new(left).into(), right: Self::new(right).into() }
    }

    pub fn push_left(&mut self, node: Node<T>) -> &mut Self {
        if let Some(left) = self.left.take() {
            (*left).borrow_mut().push_left(node);
            self.left = Some(left);
        } else {
            self.left = node.into();
        }

        self
    }

    pub fn push_right(&mut self, node: Node<T>) -> &mut Self {
        /* ?? not workinf
        if let Some(right) = &self.right {
            return right.borrow_mut().push_right(node);
        } else {
            self.left = node.into();
        }
        */

        if let Some(right) = self.right.take() {
            (*right).borrow_mut().push_right(node);
            self.right = Some(right);
        } else {
            self.right = node.into();
        }

        self
    }

    pub fn push(&mut self, left: Node<T>, right: Node<T>) -> &mut Self {
        self.push_left(left);
        self.push_right(right);
        self
    }

    /*
    pub fn push_binary(&mut self, node: Node<T>) -> &mut Self {
        if node.value <= self.value {
            if let Some(v) = self.left.take() {
                (*v).borrow_mut().push_binary(node); // !!! not *v.borrow_mut().push(value)
                self.left = Some(v); // must return self.left
            } else {
                self.left = node.into();
            }
        } else {
            if let Some(v) = self.right.take() {
                (*v).borrow_mut().push_binary(node);
                self.right = Some(v);
            } else {
                self.right = node.into();
            }
        }

        self
    }
    */

    fn count_help(&self, size: &mut usize) {
        if let Some(left) = &self.left {
            left.borrow().count_help(size);
            *size += 1;
        }

        if let Some(right) = &self.right {
            right.borrow().count_help(size);
            *size += 1;
        }
    }

    pub fn count(&self) -> usize {
        let mut result = 0;

        if let Some(left) = &self.left {
            left.borrow().count_help(&mut result);
            result += 1;
        }

        if let Some(right) = &self.right {
            right.borrow().count_help(&mut result);
            result += 1;
        }

        result + 1
    }

    pub fn levels(&self) -> usize {
        // println!("~~~ {:?}, {}, {}", self.value, self.left.is_some(), self.right.is_some());

        let left_levels = match &self.left {
            Some(v) => v.borrow().levels(),
            None => 0,
        };

        let right_levels = match &self.right {
            Some(v) => v.borrow().levels(),
            None => 0,
        };

        max(left_levels, right_levels) + 1
    }
}
