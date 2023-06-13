use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(PartialEq, Clone)]
pub struct LinkedList<T> {
    header: Next<T>,
    size: usize,
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

    pub fn into_next(self) -> Next<T> {
        Some(Rc::new(RefCell::new(self)))
    }
}

impl<T: Debug + Clone + Copy + PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        Self { header: None, size: 0 }
    }

    pub fn push(&mut self, value: T) -> &mut Self {
        let mut next = self.header.clone();
        // println!("~~> push value: {:?}", value);

        if next.is_none() {
            self.size += 1;
            self.header = Node::new(value).into_next();
            return self;
        }

        while let Some(node) = next.clone() {
            let mut node = node.borrow_mut();
            if node.value == value {
                // avoid duplicate values
                return self;
            }

            match &node.next {
                Some(v) => next = Some(v.clone()),
                None => {
                    node.next = Node::new(value).into_next();
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
        let mut vec = Vec::with_capacity(self.size);
        let mut next = self.header.clone();

        while let Some(node) = next.clone() {
            vec.push(node.borrow().value);
            next = node.borrow().next.clone();
        }

        vec
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

    pub fn walk(next: Next<T>, steps: usize) -> Next<T> {
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

    pub fn has_cycle(&self) -> (Next<T>, bool) {
        let mut slow = self.header.clone();
        let mut fast = self.header.clone();

        loop {
            match fast {
                None => return (None, false),
                Some(_) => fast = Self::walk(fast, 2),
            }

            slow = Self::walk(slow, 1);

            match fast.clone() {
                None => return (None, false),
                Some(v) => {
                    if v.borrow().value == slow.clone().unwrap().borrow().value {
                        return (fast, true);
                    }
                }
            }
        }
    }

    pub fn cycle_start(&self) -> Next<T> {
        let (mut fast, has_cycle) = self.has_cycle();
        if !has_cycle {
            return None;
        }

        let mut slow = self.header.clone();

        loop {
            let (v1, v2) = match (slow.clone(), fast.clone()) {
                (Some(v1), Some(v2)) => (v1, v2),
                _ => return None,
            };

            if v1.borrow().value == v2.borrow().value {
                return slow;
            }

            slow = Self::walk(slow, 1);
            fast = Self::walk(fast, 1);
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
        assert_eq!(list.get(0).unwrap().borrow().value, 1);
        assert_eq!(list.get(2).unwrap().borrow().value, 3);
        assert_eq!(list.last().unwrap().borrow().value, 5);

        let n2 = list.get(1);
        let last = list.last();
        assert_eq!(LinkedList::walk(n2.clone(), 2).unwrap().borrow().value, 4);

        last.unwrap().borrow_mut().next = n2;
        let (next, ok) = list.has_cycle();
        assert!(ok);
        assert_eq!(next.unwrap().borrow().value, 5);

        assert_eq!(list.cycle_start().unwrap().borrow().value, 2);
    }
}
