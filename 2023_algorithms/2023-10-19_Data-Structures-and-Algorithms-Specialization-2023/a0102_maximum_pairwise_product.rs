use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter size of numbers:");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut num: usize = input.trim().parse().expect("Input not an integer");

    if num < 2 {
        panic!("invalid size");
    }

    let mut numbers: Vec<usize> = Vec::with_capacity(num);

    for _ in 0..num {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        num = input.trim().parse().expect("Input not an integer");
        numbers.push(num);
    }

    println!("ans: {}", maximum_pairwise_product(&numbers));
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

    numbers.iter().enumerate().for_each(|(i, v)| {
        if i == idx1.0 {
            return;
        }

        if !idx2.1 || v > &numbers[idx2.0] {
            idx2.0 = i;
            idx2.1 = true;
        }
    });

    (numbers[idx1.0] as u128) * (numbers[idx2.0] as u128)
}
