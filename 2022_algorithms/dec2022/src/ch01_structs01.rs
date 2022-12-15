#![allow(dead_code)]
use std::any::type_name;

fn type_name_of<T: ?Sized>(_: &T) -> String {
    format!("{}", type_name::<T>())
}

///
#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(size: usize) -> Self {
        Self { data: Vec::with_capacity(size) }
    }

    pub fn push(&mut self, item: T) -> &mut Self {
        self.data.push(item);
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.data.get(idx)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn peek(&self) -> Option<&T> {
        match self.data.len() {
            0 => None,
            v => self.data.get(v - 1),
        }
    }
}

///
#[derive(Debug, Clone)]
struct Queue<T> {
    size: usize,
    data: Vec<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new(size: usize) -> Self {
        Self { size: 0, data: Vec::with_capacity(size) }
    }

    fn update(&mut self) {
        let len = self.data.len();
        if len > 8 && self.size < len / 2 {
            self.data = self.data[len - self.size..].to_vec();
        }
    }

    pub fn size_and_len(&self) -> (usize, usize) {
        (self.size, self.data.len())
    }

    pub fn push(&mut self, item: T) -> &mut Self {
        self.data.push(item);
        self.size += 1;
        self.update();
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let len = self.data.len();
        let val = Some(self.data[len - self.size].clone());
        self.size -= 1;
        self.update();
        val
    }
}

///
#[derive(PartialEq, Debug)]
struct LinkedNode<T> {
    val: T,
    next: Option<Box<LinkedNode<T>>>,
}

// std::collections::LinkedList;
#[derive(PartialEq, Debug)]
struct LinkedList<T> {
    header: Option<Box<LinkedNode<T>>>,
}

impl<T> LinkedNode<T> {
    fn new(val: T) -> Self {
        Self { val, next: None }
    }
}

impl<T: PartialEq> LinkedList<T> {
    fn new() -> Self {
        Self { header: None }
    }

    fn push_front(&mut self, val: T) -> &mut Self {
        let mut val = LinkedNode::new(val);
        let header = self.header.take();
        val.next = header;
        self.header = Some(Box::new(val));
        self
    }

    fn pop_front(&mut self) -> Option<LinkedNode<T>> {
        let mut val: LinkedNode<T> = match self.header.take() {
            Some(v) => *v,
            None => return None,
        };

        self.header = val.next.take();
        Some(val)
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut curr = self.header.take();

        while let Some(mut v) = curr.take() {
            let next = v.next.take();
            v.next = prev.take();
            prev = Some(v);
            curr = next;
        }

        self.header = prev.take();
    }

    fn find(&self, item: LinkedNode<T>) -> Option<usize> {
        let mut curr: &LinkedNode<T> = match &(self.header) {
            None => return None,
            Some(v) => v,
        };

        if curr == &item {
            return Some(0);
        }

        let item = Box::new(item);
        let mut ans: usize = 0;

        while let Some(v) = &(curr.next) {
            ans += 1;
            if *v == item {
                return Some(ans);
            }
            curr = v;
        }

        None
    }

    fn len(&self) -> usize {
        let mut val: &LinkedNode<T> = match &(self.header) {
            None => return 0,
            Some(v) => v,
        };

        let mut size: usize = 1;

        while let Some(ref v) = &(val.next) {
            size += 1;
            val = v;
        }
        size
    }

    pub fn is_palindrome(head: Option<Box<LinkedNode<T>>>) -> bool {
        let mut stack = Vec::new();

        let mut curr = match head {
            None => return true,
            Some(v) => v,
        };

        loop {
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
}

#[cfg(test)]
mod tests {
    // $ cargo test --lib -- --show-output arraries
    #[test]
    fn arraries() {
        let arr = [1, 2, 3, 4, 5, 6, 7];
        dbg!(&super::type_name_of(&arr)); // [i32; 3]

        let slice = &arr[3..];
        dbg!(&super::type_name_of(&slice)); // &[i32]

        let mut vec = vec![1, 2, 3, 4];
        dbg!(&super::type_name_of(&vec)); // Vec<i32>

        let slice = &vec[3..];
        dbg!(&super::type_name_of(&slice)); // &[i32]

        vec.push(5);
        dbg!(&vec);

        let _ = vec.pop();
        dbg!(&vec);

        vec.insert(0, 0);
        dbg!(&vec);

        let _ = vec.remove(0);
        dbg!(&vec);
    }

    // cargo test --lib -- --show-output stack
    #[test]
    fn stack() {
        let mut stack = super::Stack::new(4);
        stack.push(1).push(2);

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.get(0), Some(&1));
        assert_eq!(stack.get(1), None);
    }

    // cargo test --lib -- --show-output queue
    #[test]
    fn queue() {
        let mut queue = super::Queue::new(4);
        assert_eq!(queue.size_and_len(), (0, 0));

        queue.push(1).push(2);
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.size_and_len(), (0, 2));

        queue.push(3).push(4);
    }

    // cargo test --lib -- --show-output ch01_structs01::tests::linked_list
    #[test]
    fn linked_list() {
        let mut list = super::LinkedList::new();
        list.push_front(3).push_front(2).push_front(1);
        dbg!(&list);

        assert_eq!(list.len(), 3);

        assert_eq!(list.pop_front(), Some(super::LinkedNode::new(1)));
        assert_eq!(list.len(), 2);

        assert_eq!(list.find(super::LinkedNode::new(1)), None);
        assert_eq!(list.find(super::LinkedNode::new(3)), Some(1));
    }
}
