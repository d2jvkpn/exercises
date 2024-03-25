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
    let mut idx1 = (0, false);
    let mut idx2 = (0, false);

    numbers.iter().enumerate().for_each(|(i, v)| {
        if !idx1.1 || v > &numbers[idx1.0] {
            idx1.0 = i;
            idx1.1 = true;
        }
    });

    numbers.iter().enumerate().filter(|(i, _)| *i != idx1.0).for_each(|(i, v)| {
        if !idx2.1 || v > &numbers[idx2.0] {
            idx2.0 = i;
            idx2.1 = true;
        }
    });

    (numbers[idx1.0] as u128) * (numbers[idx2.0] as u128)
}
