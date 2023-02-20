fn greatest_common_divisor(num1: usize, num2: usize) -> usize {
    let (mut min, mut max) = (num1.min(num2), num1.max(num2));

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_greatest_common_divisor() {
        assert_eq!(greatest_common_divisor(78, 52), 26);
        assert_eq!(greatest_common_divisor(6, 20), 2);
        assert_eq!(greatest_common_divisor(7, 5), 1);
    }
}
