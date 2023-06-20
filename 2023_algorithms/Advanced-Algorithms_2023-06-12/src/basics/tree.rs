use std::{cell::RefCell, cmp::max, fmt::Debug, rc::Rc};

#[derive(Debug)]
pub struct Tree<T> {
    pub root: Child<T>,
    size: usize,
}

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

    pub fn push_left(&mut self, node: Node<T>) {
        if let Some(left) = self.left.take() {
            (*left).borrow_mut().push_left(node);
            self.left = Some(left);
        } else {
            self.left = node.into();
        }
    }

    pub fn push_right(&mut self, node: Node<T>) {
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
    }

    pub fn push(&mut self, left: Node<T>, right: Node<T>) {
        self.push_left(left);
        self.push_right(right);
    }

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

    pub fn count(&self) -> (usize, usize) {
        let mut result = (0, 0);

        if let Some(left) = &self.left {
            left.borrow().count_help(&mut result.0);
            result.0 += 1;
        }

        if let Some(right) = &self.right {
            right.borrow().count_help(&mut result.1);
            result.1 += 1;
        }

        result
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

impl<T: PartialEq + PartialOrd + Debug + Clone> Tree<T> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push_left(&mut self, node: Node<T>) {
        let count = node.count();

        match &self.root {
            Some(root) => return root.borrow_mut().push_left(node),
            None => self.root = node.into(),
        }

        self.size += count.0 + count.1 + 1;
    }

    pub fn push_right(&mut self, node: Node<T>) {
        let count = node.count();

        match &self.root {
            Some(root) => return root.borrow_mut().push_right(node),
            None => self.root = node.into(),
        }

        self.size += count.0 + count.1 + 1;
    }

    pub fn levels(&self) -> usize {
        match &self.root {
            Some(root) => root.borrow().levels(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_tree() {
        let mut tree = Tree::new();

        let mut n1 = Node::new(1);
        let mut n2 = Node::new(2);
        let mut n3 = Node::new(3);

        n2.push(Node::new(4), Node::new(5));
        n3.push(Node::new(6), Node::new(7));

        n1.push(n2, n3);
        assert_eq!(n1.count(), (3, 3));
        tree.push_left(n1);
        dbg!(&tree);

        assert_eq!(tree.size(), 7);
        assert_eq!(tree.levels(), 3);
    }
}
