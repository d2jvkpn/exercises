#![allow(dead_code)]
use std::collections::HashMap;

mod c0301_stack;

use c0301_stack::*;

fn main() {
    assert!(par_checker("()(())"));
    assert!(!par_checker("()((&)"));

    let infix = "( A + B ) * ( C + D )";
    let postfix = infix_to_postfix(infix);
    assert_eq!(postfix, Some("A B + C D + *".to_string()));
    println!("{} => {:?}", infix, postfix);
}

fn infix_to_postfix(infix: &str) -> Option<String> {
    if !par_checker(infix) {
        return None;
    }

    let perc = HashMap::from([('(', 1), (')', 1), ('+', 2), ('-', 2), ('*', 3), ('/', 4)]);
    // println!("!!! {:?}, {}", perc.get("("), perc["("]);

    let mut op_stack = Stack::new();
    let mut postfix = Vec::new();

    for token in infix.chars() {
        // dbg!((&token, &op_stack, &postfix));
        if token == ' ' {
            continue;
        }

        if token >= 'A' && token <= 'Z' {
            // symbol
            postfix.push(token);
        } else if token >= '0' && token <= '9' {
            // number
            postfix.push(token);
        } else if token == '(' {
            op_stack.push('(');
        } else if token == ')' {
            // operators
            let mut top = op_stack.pop()?;
            while top != '(' {
                postfix.push(top);
                top = op_stack.pop()?;
            }
        } else {
            while !op_stack.is_empty() && perc[op_stack.peek().unwrap()] >= perc[&token] {
                postfix.push(op_stack.pop().unwrap());
            }

            op_stack.push(token);
        }
    }

    while let Some(v) = op_stack.pop() {
        postfix.push(v);
    }

    // dbg!(&postfix);
    let ans: Vec<String> = postfix.into_iter().map(|v| v.to_string()).collect();
    Some(ans.join(" "))
}

fn par_checker(par: &str) -> bool {
    fn par_match(a: char, b: char) -> bool {
        (a, b) == ('(', ')')
    }

    let chars: Vec<char> = par.chars().collect();
    let mut stack: Stack<char> = Stack::with_capacity(par.len());
    let mut c;

    for i in 0..chars.len() {
        c = chars[i];

        match c {
            '(' => {
                let _ = stack.push(c);
            }
            ')' => {
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
