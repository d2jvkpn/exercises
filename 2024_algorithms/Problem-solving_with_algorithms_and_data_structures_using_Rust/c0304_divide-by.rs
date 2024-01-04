#![allow(dead_code)]

mod c0301_stack;

use c0301_stack::*;

fn main() {
    println!("divide_by(102, 2) = b{}", divide_by(102, 2));
    println!("divide_by(102, 8) = x{}", divide_by(102, 8));
    println!("divide_by(102, 16) = x{}", divide_by(102, 16));
}

pub fn divide_by(mut num: u32, base: u32) -> String {
    let digitals = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    if base > digitals.len() as u32 {
        panic!("unsupport base value");
    }

    let mut rem = Stack::new();
    let mut val;

    while num > 0 {
        val = num % base;
        rem.push(val);
        num /= base;
    }

    let mut bin = "".to_string();
    while let Some(v) = rem.pop() {
        bin += digitals[v as usize].to_string().as_str();
    }

    bin
}
