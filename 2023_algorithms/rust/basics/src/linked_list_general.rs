use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(PartialEq, Clone)]
pub struct LinkedList<T> {
    pub header: Next<T>,
    size: usize,
    pub tail: Next<T>,
}

#[derive(PartialEq, Clone)]
pub struct Node<T> {
    pub value: T,
    pub next: Next<T>,
}

pub type Next<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T: PartialEq + Clone> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

impl<T> From<Node<T>> for Next<T> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

impl<T: Debug + Clone + Copy + PartialEq> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug + Clone + Copy + PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        Self { header: None, size: 0, tail: None }
    }

    // time complexity: O(1)
    pub fn push(&mut self, value: T) -> &mut Self {
        self.size += 1;

        let node = Rc::new(RefCell::new(Node::new(value)));

        let header = match &self.header {
            None => {
                self.header = Some(node);
                return self;
            }
            Some(v) => v,
        };

        match &self.tail {
            None => header.borrow_mut().next = Some(node.clone()),
            Some(v) => v.borrow_mut().next = Some(node.clone()),
        }

        self.tail = Some(node);

        self
    }

    // time complexity: O(1)
    pub fn count(&mut self, value: T) -> usize {
        let mut count = 0;

        let mut next = self.header.clone();

        while let Some(node) = next.clone() {
            let node = node.borrow();
            if node.value == value {
                count += 1;
            }

            next = node.next.clone();
        }

        count
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn as_vec(&self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.size);
        let mut next = self.header.clone();

        while let Some(node) = next.clone() {
            vec.push(node.borrow().value);
            next = node.borrow().next.clone();
        }

        vec
    }

    pub fn first(&self, idx: usize) -> Next<T> {
        let mut next = self.header.clone();
        let mut count = 0;

        loop {
            if count == idx {
                return next;
            }

            match next.clone() {
                None => return None,
                Some(v) => next = v.borrow().next.clone(),
            }
            count += 1;
        }
    }

    pub fn walk(next: &Next<T>, steps: usize) -> Next<T> {
        let mut current = next.clone();
        let mut count = 0;

        loop {
            if count == steps {
                return current;
            }

            match current.clone() {
                None => return None,
                Some(v) => current = v.borrow().next.clone(),
            }
            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_linked_list() {
        let mut list = LinkedList::new();
        list.push(1).push(2).push(3).push(2).push(4).push(5);

        assert_eq!(list.count(2), 2);
        assert_eq!(list.size(), 6);
        assert_eq!(list.first(0).unwrap().borrow().value, 1);
        assert_eq!(list.first(2).unwrap().borrow().value, 3);
    }
}
