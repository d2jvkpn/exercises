use std::{cell::RefCell, cmp::max, fmt::Debug, rc::Rc};

pub type Child<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node<T> {
    pub data: T,
    pub left: Child<T>,
    pub right: Child<T>,
}

impl<T> From<Node<T>> for Child<T> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

impl<T> From<Node<T>> for Rc<RefCell<Node<T>>> {
    fn from(node: Node<T>) -> Self {
        Rc::new(RefCell::new(node))
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        eprintln!("==> drop Node.");
    }
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, left: None, right: None }
    }

    pub fn triangle(data: T, left: T, right: T) -> Self {
        Self { data, left: Self::new(left).into(), right: Self::new(right).into() }
    }

    pub fn set_children(&mut self, left: Node<T>, right: Node<T>) {
        self.left = left.into();
        self.right = right.into();
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
        /* ?? not working
        if let Some(right) = &self.right {
            return right.borrow_mut().push_right(node);
        } else {
            self.right = node.into();
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

    pub fn push_binary(&mut self, node: Node<T>) -> &mut Self {
        if node.data <= self.data {
            if let Some(v) = self.left.take() {
                (*v).borrow_mut().push_binary(node); // !!! not *v.borrow_mut().push(data)
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

    fn count_recur(&self, ans: &mut usize) {
        if let Some(left) = &self.left {
            left.borrow().count_recur(ans);
            *ans += 1;
        }

        if let Some(right) = &self.right {
            right.borrow().count_recur(ans);
            *ans += 1;
        }
    }

    pub fn count(&self) -> usize {
        let mut ans = 0;

        if let Some(left) = &self.left {
            left.borrow().count_recur(&mut ans);
            ans += 1;
        }

        if let Some(right) = &self.right {
            right.borrow().count_recur(&mut ans);
            ans += 1;
        }

        ans + 1
    }

    pub fn height(&self) -> usize {
        // println!("~~~ {:?}, {}, {}", self.data, self.left.is_some(), self.right.is_some());

        let h1 = match &self.left {
            Some(v) => v.borrow().height(),
            None => 0,
        };

        let h2 = match &self.right {
            Some(v) => v.borrow().height(),
            None => 0,
        };

        max(h1, h2) + 1
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}
