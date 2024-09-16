use std::cmp::max;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    let mut val;

    print!("==> Enter size of numbers: ");

    let _ = stdout().flush();
    input.clear();
    stdin().read_line(&mut input).expect("Failed to read line");
    let num: usize = input.trim().parse().expect("Input not an integer");

    if num < 2 {
        panic!("invalid size: {}", num);
    }

    let mut numbers: Vec<usize> = Vec::with_capacity(num);

    for _ in 0..num {
        input.clear();
        stdin().read_line(&mut input).expect("Failed to read line");
        val = input.trim().parse().expect("Input not an integer");
        numbers.push(val);
    }

    println!("==> Ans: {}", maximum_pairwise_product(&numbers));
}

fn maximum_pairwise_product(numbers: &Vec<usize>) -> u128 {
    let mut ans = 0;
    dbg!(&numbers);

    if numbers.len() < 2 {
        return 0;
    }

    numbers.iter().enumerate().for_each(|(i, v1)| {
        numbers[i + 1..].iter().for_each(|v2| {
            ans = max(ans, (*v1 as u128) * (*v2 as u128));
        });
    });

    ans
}
