use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub type Child<T> = Option<Rc<RefCell<QNode<T>>>>;

#[derive(PartialEq, Debug, Clone)]
pub struct QNode<T> {
    pub value: T,
    pub next: Child<T>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Queue<T> {
    pub header: Child<T>,
    pub tail: Child<T>,
    size: usize,
}

impl<T: Debug + Clone + PartialEq> QNode<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }

    pub fn new_child(value: T) -> Child<T> {
        Some(Rc::new(RefCell::new(Self::new(value))))
    }
}

impl<T: Debug + Clone + PartialEq> Queue<T> {
    pub fn new() -> Self {
        Queue { header: None, tail: None, size: 0 }
    }

    pub fn new_with(value: T) -> Self {
        let header = QNode::new_child(value);
        Queue { header: header, tail: None, size: 1 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, value: T) -> &mut Self {
        let node = QNode::new_child(value);
        self.size += 1;

        let header = match &self.header {
            None => {
                self.header = node;
                return self;
            }
            Some(v) => v,
        };

        match &self.tail {
            None => header.borrow_mut().next = node.clone(),
            Some(v) => v.borrow_mut().next = node.clone(),
        }

        self.tail = node;
        return self;
    }

    pub fn pop(&mut self) -> Child<T> {
        let header = match self.header.take() {
            None => return None,
            Some(v) => v,
        };

        self.size -= 1;
        self.header = header.borrow_mut().next.take();
        return Some(header);
    }

    pub fn as_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut child = self.header.clone();

        while let Some(node) = child {
            vec.push(node.borrow().value.clone());
            child = node.borrow().next.clone();
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_queue() {
        let mut queue = Queue::new_with(1);
        queue.push(2).push(3).push(4);

        assert_eq!(queue.size(), 4);
        assert_eq!(queue.pop(), QNode::new_child(1));
        assert_eq!(queue.size(), 3);
        assert_eq!(queue.as_vec(), vec![2, 3, 4]);

        let mut queue = Queue::new_with(1);
        assert!(queue.tail.is_none());
        assert_eq!(queue.pop(), QNode::new_child(1));
        assert!(queue.header.is_none());
    }
}