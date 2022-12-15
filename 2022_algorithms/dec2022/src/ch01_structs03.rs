#![allow(dead_code)]

use std::{cell::RefCell, cmp::PartialEq, fmt::Debug, rc::Rc};

#[derive(Debug)]
pub struct Node<T: Debug + PartialEq> {
    pub val: T,
    pub left: Option<Rc<RefCell<Node<T>>>>,
    pub right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug + PartialEq> Node<T> {
    pub fn new(val: T) -> Self {
        Self { val, left: None, right: None }
    }

    pub fn as_rc(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { val, left: None, right: None }))
    }

    pub fn set_children(&mut self, left: Option<T>, right: Option<T>) {
        if let Some(v) = left {
            self.left = Some(Node::as_rc(v));
        }

        if let Some(v) = right {
            self.right = Some(Node::as_rc(v));
        }
    }
}

#[derive(Debug)]
pub struct Tree<T: Debug + PartialEq> {
    header: Rc<RefCell<Node<T>>>,
}

impl<T: Debug + PartialEq> Tree<T> {
    pub fn new(val: T) -> Self {
        Self { header: Node::as_rc(val) }
    }
}

pub fn slice2heap<T: Debug + PartialEq + Clone>(node: Rc<RefCell<Node<T>>>, mut slice: &[T]) {
    if slice.is_empty() {
        return;
    }

    if node.borrow().left.is_none() {
        if slice.len() > 0 {
            node.borrow_mut().left = Some(Node::as_rc(slice[0].clone()));
            slice = &slice[1..];
        } else {
            return;
        }
    }

    if node.borrow().right.is_none() {
        if slice.len() > 0 {
            node.borrow_mut().right = Some(Node::as_rc(slice[0].clone()));
            slice = &slice[1..];
        } else {
            return;
        }
    }

    let (front, end) = slice.split_at((slice.len() + 1) / 2);
    // dbg!(&front);
    // dbg!(&end);

    slice2heap(node.clone().borrow_mut().left.as_ref().unwrap().clone(), front);
    slice2heap(node.clone().borrow_mut().right.as_ref().unwrap().clone(), end);
}

// https://www.jianshu.com/p/7a62dcc96304
fn in_order<T: Debug + PartialEq>(node: &Option<Rc<RefCell<Node<T>>>>) {
    let node = if let Some(v) = node { v } else { return };

    in_order(&node.borrow().left);
    println!("~~~ {:?}", node.borrow().val);
    in_order(&node.borrow().right);
}

fn post_order<T: Debug + PartialEq>(node: &Option<Rc<RefCell<Node<T>>>>) {
    let node = if let Some(v) = node { v } else { return };

    post_order(&node.borrow().left);
    post_order(&node.borrow().right);
    println!("~~~ {:?}", node.borrow().val);
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice2tree() {
        let mut vec = vec![2, 3, 4, 5, 6, 7, 8, 9];
        let tree = super::Tree::new(1);
        super::slice2heap(tree.header.clone(), &mut vec);

        super::in_order(&Some(tree.header.clone()));
        println!("=====");
        super::post_order(&Some(tree.header.clone()));
        // dbg!(&tree);
    }
}
