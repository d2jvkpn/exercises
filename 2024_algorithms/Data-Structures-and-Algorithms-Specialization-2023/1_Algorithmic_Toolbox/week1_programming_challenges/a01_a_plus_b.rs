use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();

    print!("==> Enter n1 and n2: ");
    let _ = stdout().flush();

    input.clear();
    stdin().read_line(&mut input).expect("Failed to read line");
    let n1: i32 = input.trim().parse().expect("Input not an integer");

    input.clear();
    stdin().read_line(&mut input).expect("Failed to read line");
    let n2: i32 =
        input.trim().parse().unwrap_or_else(|_| panic!("Input not an integer: {}", input));

    println!("==> Ans: {}", n1 + n2);
}
