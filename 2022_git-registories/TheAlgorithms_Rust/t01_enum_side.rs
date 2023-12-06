#![allow(dead_code)]

use std::ops::Not;

/// Refers to the left or right subtree of an `AVLNode`.
#[derive(Clone, Copy, Debug)]
enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        use Side::*;

        match self {
            Left => Right,
            Right => Left,
        }
    }
}

fn main() {
    // println!("Hello, wrold!");
    let left = Side::Left;

    println!("{:?}, {:?}", left, !left);
}
