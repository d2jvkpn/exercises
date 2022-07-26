#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    next: Option<Rc<Node<T>>>,
    prev: RefCell<Option<Weak<Node<T>>>>,
    data: T,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, data: T) -> Self {
        let new_node =
            Rc::new(Node { data: data, next: self.head.clone(), prev: RefCell::new(None) });

        if let Some(node) = self.head.clone() {
            let mut prev = node.prev.borrow_mut();
            *prev = Some(Rc::downgrade(&new_node));
        }

        LinkedList { head: Some(new_node) }
    }
}

fn main() {
    let list_of_nums = LinkedList::new().append(1).append(2).append(3);
    println!("nums: {:?}", list_of_nums);
}
