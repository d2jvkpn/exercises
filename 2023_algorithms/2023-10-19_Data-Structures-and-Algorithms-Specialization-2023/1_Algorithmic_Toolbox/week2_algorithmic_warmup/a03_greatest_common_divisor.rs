mod utils;

/*
gcd(a, b) = gcd(a', b) = gcd(a, b')
a' + b*n = a
n*a + b' = b

3918848, 1653264
*/
fn main() {
    let a: usize = utils::read_usize("==> Enter number a: ").unwrap();
    let b: usize = utils::read_usize("==> Enter number b: ").unwrap();

    println!("==> Ans: {}", gdc_rec(a, b));
    println!("==> Ans: {}", gdc(a, b));
}

fn gdc_rec(a: usize, b: usize) -> usize {
    println!("~~~ a={a}, b={b}");
    if b == 0 {
        return a;
    }

    return gdc_rec(b, a % b);
}

fn gdc(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn t_gcd() {
        assert_eq!(gdc_rec(3918848, 1653264), 61232);
    }
}
