#![allow(dead_code)]

mod c0301_stack;

use c0301_stack::*;

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
