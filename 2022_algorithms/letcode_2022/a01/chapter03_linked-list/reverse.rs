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

type Link = Option<Box<ListNode>>;

#[derive(Debug)]
struct List {
    head: Link,
}

impl List {
    pub fn new(node: ListNode) -> Self {
        List {
            head: Some(Box::new(node)),
        }
    }

    pub fn push_front(&mut self, val: i32) -> &mut Self {
        self.head = Some(Box::new(ListNode {
            val,
            next: self.head.take(),
        }));
        self
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut tmp) = current.take() {
            let next = tmp.next.take();
            tmp.next = prev.take();
            prev = Some(tmp);
            current = next;
        }

        self.head = prev.take();
    }
}

fn reverse(list: &mut List) {
    list.reverse()
}

fn main() {
    let mut list = List::new(ListNode::new(5));
    list.push_front(4).push_front(3).push_front(2).push_front(1);

    println!("{:?}", list);
    // list.reverse();
    reverse(&mut list);
    println!("{:?}", list);
}
