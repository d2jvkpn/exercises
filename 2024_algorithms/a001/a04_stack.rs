#![allow(dead_code)]

fn main() {
    assert!(par_checker("()(())"));
    assert!(!par_checker("()((&)"));

    assert!(par_checker("(){}[]"));
    assert!(par_checker("{()}[]"));
    assert!(!par_checker("(){)[}"));
}

fn par_checker(par: &str) -> bool {
    let chars: Vec<char> = par.chars().collect();
    let mut stack: Stack<char> = Stack::with_capacity(par.len());
    let mut c;

    for i in 0..chars.len() {
        c = chars[i];

        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                if stack.is_empty() {
                    return false;
                }

                let top = match stack.pop() {
                    Some(v) => v,
                    None => return false,
                };

                if !par_match(top, c) {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

fn par_match(a: char, b: char) -> bool {
    match (a, b) {
        ('(', ')') | ('[', ']') | ('{', '}') => true,
        _ => false,
    }
}

#[derive(Debug)]
struct Stack<T> {
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

    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.height += 1;
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
