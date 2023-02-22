use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value: value, next: None, prev: None }
    }

    fn into_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

#[derive(Clone)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T: Clone> LinkedList<T> {
    fn new() -> Self {
        Self { head: None, tail: None, size: 0 }
    }

    pub fn append(&mut self, v: T) -> &mut Self {
        let new = Node::new(v).into_rc();

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
        Self { head: item.head.clone(), tail: item.tail.clone() }
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
                result = Some(current.value.clone());
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
                result = Some(current.value.clone());
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

    #[test]
    fn t_linked_list() {
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
