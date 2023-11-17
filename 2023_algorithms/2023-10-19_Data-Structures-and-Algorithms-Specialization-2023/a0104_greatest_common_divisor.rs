use std::io::{stdin, stdout, Write};

/*
gcd(a, b) = gcd(a', b) = gcd(a, b')
a' + b*n = a
n*a + b' = b

3918848, 1653264
*/
fn main() {
    let mut input = String::new();

    print!("==> Enter two numbers: ");

    let _ = stdout().flush();
    input.clear();
    stdin().read_line(&mut input).expect("Failed to read line");
    let a: usize = input.trim().parse().expect("Input not an integer");

    input.clear();
    stdin().read_line(&mut input).expect("Failed to read line");
    let b: usize = input.trim().parse().expect("Input not an integer");

    println!("==> ans: {}", greatest_common_divisor(a, b));
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    println!("~~~ a={a}, b={b}");
    if b == 0 {
        return a;
    }

    return greatest_common_divisor(b, a % b);
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn t_gcd() {
        assert_eq!(greatest_common_divisor(3918848, 1653264), 61232);
    }
}
