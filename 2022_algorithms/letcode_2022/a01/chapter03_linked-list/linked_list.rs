#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[derive(Debug)]
struct List {
    head: Option<Box<ListNode>>,
}

impl List {
    pub fn empty() -> Self {
        Self { head: None }
    }

    pub fn new(node: ListNode) -> Self {
        Self { head: Some(Box::new(node)) }
    }

    pub fn push_front(&mut self, val: i32) -> &mut Self {
        self.head = Some(Box::new(ListNode { val, next: self.head.take() }));
        self
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut curr = self.head.take();

        while let Some(mut tmp) = curr.take() {
            let next = tmp.next.take();
            tmp.next = prev.take();
            prev = Some(tmp);
            curr = next;
        }

        self.head = prev.take();
    }
}

//fn reverse(list: &mut List) {
//    list.reverse()
//}

fn main() {
    let mut list = List::new(ListNode::new(5));
    list.push_front(4).push_front(3).push_front(2).push_front(1);

    println!("{:?}", list);
    // list.reverse();
    reverse(&mut list);
    println!("{:?}", list);
}
