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

impl<T> From<Node<T>> for Next<T> {
    fn from(node: Node<T>) -> Next<T> {
        Some(Rc::new(RefCell::new(node)))
    }
}

impl<T: Debug + Clone + PartialEq> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

/**
Queue stats:
1. {header: None, tail: None }
2. {header: Some(Item), tail: None }
3. {header: Some(Item)...., tail: Some(Item) }
*/
impl<T: Debug + Clone + PartialEq> Node<T> {
    pub fn new(item: T) -> Self {
        Self { item, next: None }
    }
}

impl<T: Debug + Clone + PartialEq> Queue<T> {
    pub fn new() -> Self {
        Queue { header: None, tail: None, size: 0 }
    }

    pub fn new_with(item: T) -> Self {
        let node = Node::new(item).into();
        Queue { header: node, tail: None, size: 1 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, item: T) -> &mut Self {
        let next: Next<T> = Node::new(item).into();
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

    pub fn push_vec(&mut self, items: Vec<T>) {
        items.into_iter().for_each(|v| {
            self.push(v);
        });
    }

    pub fn pop(&mut self) -> Option<T> {
        let (ans, header) = match self.header.take() {
            None => return None,
            Some(v) => {
                let h = v.borrow_mut().next.take();
                (v.borrow().item.clone(), h)
            }
        };

        self.size -= 1;
        if self.size == 1 {
            self.header = self.tail.take();
        } else {
            self.header = header;
        }

        Some(ans)
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
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.size(), 3);
        assert_eq!(queue.as_vec(), vec![2, 3, 4]);

        let mut queue = Queue::new_with(1);
        assert!(queue.tail.is_none());
        assert_eq!(queue.pop(), Some(1));
        assert!(queue.header.is_none());
    }
}
