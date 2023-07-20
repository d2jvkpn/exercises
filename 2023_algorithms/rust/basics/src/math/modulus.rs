#![allow(dead_code)]

use std::ops::{Add, AddAssign, Rem};

// % is remainder, not modulus: assert_eq!(-226 % 223, -3);
pub fn modulus<T: Add + AddAssign + Copy + Rem<Output = T>>(a: T, b: T) -> T {
    // ((a % b) + b) % b
    let mut ans = a % b;
    ans += b;
    ans % b
}

pub fn power_mod(b: i128, e: i128, m: i128) -> i128 {
    let mut ans = 1;

    let n = e % (m - 1);
    for _ in 1..=n {
        ans = modulus(ans * b, m);
    }

    ans
}
