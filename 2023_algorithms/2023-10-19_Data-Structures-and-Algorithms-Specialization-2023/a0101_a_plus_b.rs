use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();

    print!("==> Enter a and b: ");

    let _ = stdout().flush();
    input.clear();
    stdin().read_line(&mut input).expect("Failed to read line");
    let a: i32 = input.trim().parse().expect("Input not an integer");

    input.clear();
    stdin().read_line(&mut input).expect("Failed to read line");
    let b: i32 = input.trim().parse().unwrap_or_else(|_| panic!("Input not an integer: {}", input));

    println!("==> ans: {}", a + b);
}
