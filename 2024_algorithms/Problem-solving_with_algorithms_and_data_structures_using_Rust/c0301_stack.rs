#![allow(dead_code)]

fn main() {
    let mut stack = Stack::with_capacity(3);
    stack.push(1).push(2).push(3);

    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}

#[derive(Debug)]
pub struct Stack<T> {
    height: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { height: 0, data: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self { height: 0, data: Vec::with_capacity(cap) }
    }

    pub fn push(&mut self, val: T) -> &mut Self {
        self.data.push(val);
        self.height += 1;
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.height == 0 {
            return None;
        }

        self.height -= 1;
        self.data.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.height == 0
    }

    pub fn size(&self) -> usize {
        self.height
    }

    pub fn peek(&self) -> Option<&T> {
        if self.height == 0 {
            return None;
        }

        Some(&self.data[self.height - 1])
    }

    pub fn index_at(&self, i: usize) -> Option<&T> {
        if self.height < i + 1 {
            return None;
        }

        Some(&self.data[i])
    }
}
