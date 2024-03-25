mod utils;

use std::cmp::max;

fn main() {
    let a: usize = utils::read_usize("==> Enter number a: ").unwrap();
    let b: usize = utils::read_usize("==> Enter number b: ").unwrap();

    println!("==> Ans: {}", lcm(a, b));
}

fn lcm(a: usize, b: usize) -> usize {
    for ans in max(a, b)..=a * b {
        if ans % a == 0 && ans % b == 0 {
            return ans;
        }
    }

    a * b
}
