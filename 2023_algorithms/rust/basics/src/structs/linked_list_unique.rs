use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(PartialEq, Clone)]
pub struct LinkedList<T> {
    pub header: Next<T>,
    size: usize,
    // pub tail: Next<T>,
}

#[derive(PartialEq, Clone)]
pub struct Node<T> {
    pub item: T,
    pub next: Next<T>,
}

pub type Next<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T: PartialEq + Clone> Node<T> {
    pub fn new(item: T) -> Self {
        Self { item, next: None }
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
        Self { header: None, size: 0 }
    }

    pub fn push(&mut self, item: T) -> &mut Self {
        let mut next = self.header.clone();
        // println!("~~> push item: {:?}", item);

        if next.is_none() {
            self.size += 1;
            self.header = Node::new(item).into();
            return self;
        }

        while let Some(node) = next.clone() {
            let mut node = node.borrow_mut();
            if node.item == item {
                // avoid duplicate items
                return self;
            }

            match &node.next {
                Some(v) => next = Some(v.clone()),
                None => {
                    node.next = Node::new(item).into();
                    break;
                }
            }
        }

        self.size += 1;
        self
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn as_vec(&self) -> Vec<T> {
        let mut ans = Vec::with_capacity(self.size);
        let mut next = self.header.clone();

        while let Some(node) = next.clone() {
            ans.push(node.borrow().item);
            next = node.borrow().next.clone();
        }

        ans
    }

    pub fn get(&self, idx: usize) -> Next<T> {
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

    pub fn last(&self) -> Next<T> {
        match self.size {
            0 => None,
            v => self.get(v - 1),
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

        assert_eq!(list.size(), 5);
        assert_eq!(list.get(0).unwrap().borrow().item, 1);
        assert_eq!(list.get(2).unwrap().borrow().item, 3);
        assert_eq!(list.last().unwrap().borrow().item, 5);
    }
}
