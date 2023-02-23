mod fibnacci {
    // recursion
    pub fn v1(n: u32) -> u32 {
        match n {
            1 | 2 => 1,
            _ => v1(n - 1) + v1(n - 2),
        }
    }

    // tail recursion
    pub fn v2(n: u32) -> u32 {
        fn help(n: u32, sum: &mut u32) {
            match n {
                1 | 2 => *sum += 1,
                _ => {
                    help(n - 1, sum);
                    help(n - 2, sum);
                }
            }
        }

        let mut sum = 0;
        help(n, &mut sum);
        sum
    }

    // dynanic programming
    pub fn v3(n: u32) -> u32 {
        let mut dp = [1, 1];

        for i in 2..=n {
            let idx1 = (i % 2) as usize;
            let idx2 = ((i - 1) % 2) as usize;
            let idx3 = ((i - 2) % 2) as usize;
            dp[idx1] = dp[idx2] + dp[idx3];
        }

        dp[((n - 1) % 2) as usize]
    }

    // dynanic programming
    pub fn v4(n: u32) -> u32 {
        let (mut p1, mut p2) = (1, 1);

        for i in 3..=n {
            if i % 2 == 1 {
                p2 += p1;
            } else {
                p1 += p2;
            }
        }

        if n % 2 == 1 {
            p2
        } else {
            p1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_fibnacci() {
        assert_eq!(fibnacci::v1(10), 55);
        assert_eq!(fibnacci::v2(10), 55);
        assert_eq!(fibnacci::v3(10), 55);
        assert_eq!(fibnacci::v4(10), 55);
    }
}
