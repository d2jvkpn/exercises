fn reverse_int(mut x: i32) -> i32 {
    let mut res = 0;

    while x != 0 {
        if x > 0 && res > (i32::MAX - x % 10) / 10 {
            return 0;
        } else if x < 0 && res < (i32::MIN - x % 10) / 10 {
            return 0;
        }
        res = res * 10 + x % 10;
        x /= 10;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_int() {
        assert_eq!(reverse_int(123), 321);
        assert_eq!(reverse_int(-123), -321);
        println!("{}, {}", i32::MAX, -12 % 10);
        assert_eq!(reverse_int(-1534236469), 0);
        println!(">>> reverse_int: ok");
    }
}
