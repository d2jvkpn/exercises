use super::node::{Child, Node};
use crate::structs::queue_linked_list::Queue;
use std::{
    cell::RefCell,
    fmt::Debug,
    io::{stdin, stdout, Write},
    rc::Rc,
};

#[derive(Debug)]
pub struct Tree<T> {
    pub root: Child<T>,
    size: usize,
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Tree<T> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn height(&self) -> usize {
        match &self.root {
            Some(root) => root.borrow().height(),
            None => 0,
        }
    }

    pub fn push(&mut self, func: fn(&Child<T>, Node<T>), node: Node<T>) {
        func(&self.root, node)
    }
}

pub fn build_string_tree() -> Tree<String> {
    let mut queue: Queue<Rc<RefCell<Node<String>>>> = Queue::new();
    let mut tree = Tree::new();

    print!("==> Enter root(\"\" -> None, \".\" -> Done): ");

    match enter_node_string() {
        NodeFromString::None | NodeFromString::Quit => return tree,
        NodeFromString::Node(v) => {
            let root = Rc::new(RefCell::new(v));
            tree.root = Some(root.clone());
            queue.push(root);
        }
    };

    // let root = Rc::new(RefCell::new(Node::new(input.trim().to_string())));

    while let Some(item) = queue.pop() {
        let node = &item.borrow().item;

        //
        print!("==> Enter: {:?}.left: ", node.borrow().data);

        match enter_node_string() {
            NodeFromString::None => {}
            NodeFromString::Quit => break,
            NodeFromString::Node(v) => {
                let left: Rc<RefCell<Node<String>>> = v.into();
                node.borrow_mut().left = Some(left.clone());
                queue.push(left);
            }
        };

        print!("==> Enter: {:?}.right: ", node.borrow().data);

        match enter_node_string() {
            NodeFromString::None => {}
            NodeFromString::Quit => break,
            NodeFromString::Node(v) => {
                let right: Rc<RefCell<Node<String>>> = v.into();
                node.borrow_mut().right = Some(right.clone());
                queue.push(right);
            }
        };
    }

    tree
}

enum NodeFromString {
    None,
    Quit,
    Node(Node<String>),
}

fn enter_node_string() -> NodeFromString {
    if stdout().flush().is_err() {
        return NodeFromString::None;
    }

    let mut input = String::new();

    if stdin().read_line(&mut input).is_err() {
        return NodeFromString::None;
    };

    input = input.trim().to_string();

    match input.as_str() {
        "" => NodeFromString::None,
        "." => NodeFromString::Quit,
        _ => NodeFromString::Node(Node::new(input.clone()).into()),
    }
}
