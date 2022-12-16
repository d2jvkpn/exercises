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

    pub fn number(&self) -> usize {
        let mut num = 1;

        match &self.left {
            None => {}
            Some(v) => num += v.borrow().number(),
        }

        match &self.right {
            None => {}
            Some(v) => num += v.borrow().number(),
        }

        num
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
// left..., node, right...
fn in_order<T: Debug + PartialEq + Clone>(item: &Option<Rc<RefCell<Node<T>>>>) -> Vec<T> {
    let mut result = Vec::new();

    let node = if let Some(v) = item { v } else { return result };

    result.extend(in_order(&node.borrow().left));
    result.push(node.borrow().val.clone());
    result.extend(in_order(&node.borrow().right));

    result
}

pub fn in_order_v2<T: Debug + PartialEq + Clone>(item: Option<Rc<RefCell<Node<T>>>>) -> Vec<T> {
    fn traversal<T: Debug + PartialEq + Clone>(
        item: &Option<Rc<RefCell<Node<T>>>>,
        ans: &mut Vec<T>,
    ) {
        let node = if let Some(v) = item { v } else { return };
        traversal(&node.borrow().left, ans);
        ans.push(node.borrow().val.clone());
        traversal(&node.borrow().right, ans);
    }

    let mut ans = vec![];
    traversal(&item, &mut ans);
    ans
}

// the tree will be destoryed
pub fn in_order_v3<T: Debug + PartialEq + Clone>(item: Option<Rc<RefCell<Node<T>>>>) -> Vec<T> {
    let mut ans = Vec::new();
    let mut stack: Vec<Option<Rc<RefCell<Node<T>>>>> = Vec::new();
    let mut curr: Option<Rc<RefCell<Node<T>>>> = item;

    loop {
        if curr.is_some() {
            let node = curr.unwrap();
            curr = node.clone().borrow_mut().left.take();
            stack.push(Some(node.clone()));
        } else if !stack.is_empty() {
            let node = stack.pop().flatten().unwrap();
            ans.push(node.clone().borrow().val.clone());
            curr = node.clone().borrow_mut().right.take();
        } else {
            break;
        }
    }

    ans
}

fn post_order<T: Debug + PartialEq + Clone>(item: &Option<Rc<RefCell<Node<T>>>>) -> Vec<T> {
    let mut result = Vec::new();
    let node = if let Some(v) = item { v } else { return result };

    result.extend(post_order(&node.borrow().left));
    result.extend(post_order(&node.borrow().right));
    result.push(node.borrow().val.clone());

    result
}

#[cfg(test)]
mod tests {
    // cargo test --lib -- --show-output tree_01
    #[test]
    fn tree_01() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let tree = super::Tree::new(vec[0]);
        super::slice2heap(tree.header.clone(), &mut vec[1..]);
        //          1
        //      2       3
        //    4   5    7  8
        //  6         9

        assert_eq!(tree.header.borrow().number(), 9);

        println!(">>> in_order {:?}", super::in_order(&Some(tree.header.clone())));
        println!(">>> in_order_v2 {:?}", super::in_order_v2(Some(tree.header.clone())));
        // println!(">>> in_order_v3 {:?}", super::in_order_v3(Some(tree.header.clone())));
        println!(">>> post_order {:?}", super::post_order(&Some(tree.header.clone())));
        // dbg!(&tree);
    }
}
