#![allow(dead_code)]

const MAX_VALUE: i32 = 4200;

fn main() {
    let (x, y) = (2, 4);
    let sum = square_sum(x, y);
    println!("The value of Square of Sum = {}", sum);
}

fn square_sum(num1: i32, num2: i32) -> i32 {
    let ans = square(num1 + num2);
    ans
}

fn square(num: i32) -> i32 {
    num * num // num.powi(2)
}
