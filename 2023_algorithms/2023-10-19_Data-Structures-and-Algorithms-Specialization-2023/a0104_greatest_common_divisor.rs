use std::{
    error,
    io::{stdin, stdout, Write},
};

/*
gcd(a, b) = gcd(a', b) = gcd(a, b')
a' + b*n = a
n*a + b' = b

3918848, 1653264
*/
fn main() {
    let a: usize = read_number("==> Enter number a: ").unwrap();
    let b: usize = read_number("==> Enter number b: ").unwrap();

    println!("==> ans: {}", greatest_common_divisor(a, b));
}

fn read_number(prompt: &str) -> Result<usize, Box<dyn error::Error>> {
    let mut input = String::new();

    print!("{}", prompt);
    stdout().flush()?;
    input.clear();
    stdin().read_line(&mut input)?;
    let num: usize = input.trim().parse()?;

    Ok(num)
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
