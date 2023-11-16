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
    pub item: T,
    pub next: Next<T>,
}

impl<T: Debug + Clone + PartialEq> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug + Clone + PartialEq> Node<T> {
    pub fn new(item: T) -> Self {
        Self { item, next: None }
    }

    pub fn into_next(self) -> Next<T> {
        Some(Rc::new(RefCell::new(self)))
    }
}

impl<T: Debug + Clone + PartialEq> Queue<T> {
    pub fn new() -> Self {
        Queue { header: None, tail: None, size: 0 }
    }

    pub fn new_with(item: T) -> Self {
        let header = Node::new(item).into_next();
        Queue { header, tail: None, size: 1 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, item: T) -> &mut Self {
        let next = Node::new(item).into_next();
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
        let mut ans = Vec::new();
        let mut next = self.header.clone();

        while let Some(node) = next {
            ans.push(node.borrow().item.clone());
            next = node.borrow().next.clone();
        }

        ans
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
