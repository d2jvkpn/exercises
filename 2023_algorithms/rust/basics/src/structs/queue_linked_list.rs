use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub struct Queue<T> {
    pub header: Next<T>,
    pub tail: Next<T>,
    size: usize,
}

pub type Next<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(PartialEq, Debug, Clone)]
pub struct Node<T> {
    pub data: T,
    pub next: Next<T>,
}

impl<T: Debug + Clone + PartialEq> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug + Clone + PartialEq> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }

    pub fn into_next(self) -> Next<T> {
        Some(Rc::new(RefCell::new(self)))
    }
}

impl<T: Debug + Clone + PartialEq> Queue<T> {
    pub fn new() -> Self {
        Queue { header: None, tail: None, size: 0 }
    }

    pub fn new_with(data: T) -> Self {
        let header = Node::new(data).into_next();
        Queue { header, tail: None, size: 1 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, data: T) -> &mut Self {
        let next = Node::new(data).into_next();
        self.size += 1;

        let header = match &self.header {
            None => {
                self.header = next;
                return self;
            }
            Some(v) => v,
        };

        match &self.tail {
            None => header.borrow_mut().next = next.clone(),
            Some(v) => v.borrow_mut().next = next.clone(),
        }

        self.tail = next;
        self
    }

    pub fn pop(&mut self) -> Next<T> {
        let header = match self.header.take() {
            None => return None,
            Some(v) => v,
        };

        self.size -= 1;
        self.header = header.borrow_mut().next.take();
        Some(header)
    }

    pub fn as_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut next = self.header.clone();

        while let Some(node) = next {
            vec.push(node.borrow().data.clone());
            next = node.borrow().next.clone();
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_queue1() {
        let mut queue = Queue::new_with(1);
        queue.push(2).push(3).push(4);

        assert_eq!(queue.size(), 4);
        assert_eq!(queue.pop(), Node::new(1).into_next());
        assert_eq!(queue.size(), 3);
        assert_eq!(queue.as_vec(), vec![2, 3, 4]);

        let mut queue = Queue::new_with(1);
        assert!(queue.tail.is_none());
        assert_eq!(queue.pop(), Node::new(1).into_next());
        assert!(queue.header.is_none());
    }
}