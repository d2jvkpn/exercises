use std::{cell::RefCell, rc::Rc};

type Child<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> {
    item: T,
    next: Child<T>,
    prev: Child<T>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Node { item, next: None, prev: None }
    }
}

impl<T> From<Node<T>> for Rc<RefCell<Node<T>>> {
    fn from(node: Node<T>) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(node))
    }
}

impl<T> From<Node<T>> for Option<Rc<RefCell<Node<T>>>> {
    fn from(node: Node<T>) -> Option<Rc<RefCell<Node<T>>>> {
        Some(Rc::new(RefCell::new(node)))
    }
}

#[derive(Clone)]
pub struct LinkedList<T> {
    head: Child<T>,
    tail: Child<T>,
    size: usize,
}

impl<T: Clone> LinkedList<T> {
    fn new() -> Self {
        Self { head: None, tail: None, size: 0 }
    }

    pub fn append(&mut self, v: T) -> &mut Self {
        let new: Rc<RefCell<Node<T>>> = Node::new(v).into();

        match self.tail.take() {
            Some(v) => {
                v.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(v);
            }
            None => {
                self.head = Some(new.clone());
            }
        }

        self.tail = Some(new);
        self.size += 1;
        self
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Clone)]
pub struct ListIterator<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> From<LinkedList<T>> for ListIterator<T> {
    fn from(item: LinkedList<T>) -> Self {
        Self { head: item.head.clone(), tail: item.tail }
    }
}

impl<T: Clone> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.head;
        let mut result = None;

        self.head = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.item.clone());
                current.next.clone()
            }
            None => None,
        };

        result
    }
}

impl<T: Clone> DoubleEndedIterator for ListIterator<T> {
    fn next_back(&mut self) -> Option<T> {
        let current = &self.tail;
        let mut result = None;

        self.tail = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.item.clone());
                current.prev.clone()
            }
            None => None,
        };
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test -- --show-output t_dll
    #[test]
    fn t_dll() {
        let mut list = LinkedList::new();
        list.append(1).append(2).append(3).append(4).append(5).append(6).append(7);

        let a1 = ListIterator::from(list.clone());
        let vec: Vec<i32> = a1.clone().collect();
        println!("{:?}", vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7]);

        let a2 = a1.clone();
        for i in a2.take(3) {
            println!("> {}", i);
        }

        println!("{:?}", a1.rev().collect::<Vec<i32>>());
    }
}
