mod utils;

fn main() {
    let num: usize = utils::read_usize("==> Enter number: ").unwrap();

    println!("==> Ans: {}", fibnacci_rec(num));
}

fn fibnacci_rec(num: usize) -> u128 {
    if num < 2 {
        return num as u128;
    }

    return fibnacci_rec(num - 1) + fibnacci_rec(num - 2);
}
