use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a and b:");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: i32 = input.trim().parse().expect("Input not an integer");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: i32 = input.trim().parse().unwrap_or_else(|_| panic!("Input not an integer: {}", input));

    println!("ans: {}", a + b);
}
