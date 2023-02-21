mod nth_stair {
    // BigO: 2^N
    // jumps
    pub fn v1(n: usize) -> usize {
        match n {
            0..=2 => return n,
            _ => {}
        }

        v1(n - 1) + v1(n - 2)
    }

    // BigO: 2^N
    pub fn v2(n: usize) -> usize {
        fn help(n: usize, sum: &mut usize) {
            match n {
                0..=2 => {
                    *sum += n;
                    return;
                }
                _ => {}
            }

            help(n - 1, sum);
            help(n - 2, sum);
        }

        let mut sum = 0;

        help(n - 1, &mut sum);
        help(n - 2, &mut sum);
        sum
    }

    // BigO: log(N)
    pub fn v3(n: usize) -> usize {
        fn fib(n: usize, a: usize, b: usize) -> usize {
            if n <= 1 {
                return b;
            }
            fib(n - 1, b, a + b)
        }

        fib(n, 1, 1)
    }

    //  BigO: N
    pub fn v4(n: usize) -> usize {
        if n < 3 {
            return n;
        }

        let mut dp = vec![0; n + 1];
        (dp[1], dp[2]) = (1, 2);

        for i in 3..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_nth_stair() {
        use nth_stair::*;

        assert_eq!(v1(5), 8);
        assert_eq!(v2(5), 8);
        assert_eq!(v3(5), 8);
        assert_eq!(v4(5), 8);
    }
}
