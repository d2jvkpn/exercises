#![allow(unused_variables)]

fn main() {
    let age = 35;

    match age {
        1 => println!("Happy 1st Birthday!"),
        13..=19 => println!("You are a teenager!"),
        v => println!("You are {v} years old."),
    }

    // Result => Option
    let ans: Option<i64> = calculate().ok();
    println!("==> ans: {:?}", ans);

    // Option => Result
    let ans: Result<i64, String> = call().ok_or("not_found".to_string());
    println!("==> ans: {:?}", ans);

    let ans: Result<i64, String> = call().ok_or_else(|| "not_found".to_string());
    println!("==> ans: {:?}", ans);
}

fn calculate() -> Result<i64, &'static str> {
    Ok(42)
}

fn call() -> Option<i64> {
    None
}

