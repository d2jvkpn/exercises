use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct ListNode {
    value: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

fn main() {
    let node1 = Rc::new(RefCell::new(ListNode { value: 1, next: None }));
    let node2 = Rc::new(RefCell::new(ListNode { value: 2, next: Some(Rc::clone(&node1)) }));

    // Accessing and printing the value of the second node and its next node
    println!("Node2 value: {}", node2.borrow().value);

    if let Some(ref next_node) = node2.borrow().next {
        println!("Node2 points to: {:?}", next_node.borrow().value);
    };
}
