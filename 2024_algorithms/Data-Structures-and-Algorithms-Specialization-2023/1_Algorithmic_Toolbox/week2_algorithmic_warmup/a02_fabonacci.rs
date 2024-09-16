#![allow(dead_code)]

mod utils;

fn main() {
    let num: usize = utils::read_usize("==> Enter number: ").unwrap();

    // println!("==> Ans: {}", fibnacci_array(num).pop().unwrap());
    println!("==> Ans: {}", fibnacci_dp(num));
}

fn fibnacci_array(num: usize) -> Vec<u128> {
    match num {
        1 => return vec![0_u128],
        2 => return vec![0, 1],
        _ => {}
    }

    let mut vec = Vec::with_capacity(num);
    vec.push(1);
    vec.push(1);

    (2..num).for_each(|i| {
        vec.push(vec[i - 2] + vec[i - 1]);
    });

    vec
}

fn fibnacci_dp(num: usize) -> u128 {
    if num < 2 {
        return num as u128;
    }

    let mut dp = (1, 1);
    let mut val = 0;

    (2..num).for_each(|_| {
        val = dp.0 + dp.1;
        (dp.0, dp.1) = (dp.1, val);
    });

    dp.1
}
