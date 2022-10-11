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

fn reverse(list: &mut List) {
    list.reverse()
}

// https://gist.github.com/Coutlaw/e974e6b130255fa99edc6173f9008bd1
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (l, r) = match (list1, list2) {
        (None, None) => return None,
        (None, Some(v)) | (Some(v), None) => return Some(v),
        (Some(l), Some(r)) => (l, r),
    };

    let node = if l.val < r.val {
        ListNode { val: l.val, next: merge_two_lists(l.next, Some(r)) }
    } else {
        ListNode { next: merge_two_lists(Some(l), r.next), val: r.val }
    };

    Some(Box::new(node))
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut stack = Vec::new();

    // ListNode
    let mut curr = match head {
        None => return true,
        Some(v) => v,
    };

    loop {
        //        if stack.len() > 0 && stack[stack.len() - 1] == curr.val {
        //            stack.pop();
        //        } else {
        //            stack.push(curr.val);
        //        }
        stack.push(curr.val);

        curr = match curr.next {
            None => break,
            Some(v) => v,
        };
    }

    for i in 0..stack.len() / 2 {
        if stack[i] != stack[stack.len() - i - 1] {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut list = List::new(ListNode::new(5));
    list.push_front(4).push_front(3).push_front(2).push_front(1);

    println!("{:?}", list);
    // list.reverse();
    reverse(&mut list);
    println!("{:?}", list);

    let mut list = List::new(ListNode::new(1));
    list.push_front(2).push_front(2).push_front(1);
    println!("~~~ {:?}", list);
    println!("    {}", is_palindrome(list.head));

    let list = List::new(ListNode::new(1));
    println!("~~~ {:?}", list);
    println!("    {}", is_palindrome(list.head));
}
