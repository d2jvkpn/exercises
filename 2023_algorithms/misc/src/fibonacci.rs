mod fibonacci {
    pub fn v1(n: usize) -> usize {
        match n {
            0 => return 0,
            1 => return 1,
            _ => return v1(n - 1) + v1(n - 2),
        }
    }

    pub fn v2(n: usize, ans: &mut usize) {
        match n {
            0 => *ans += 0,
            1 => *ans += 1,
            _ => {
                v2(n - 1, ans);
                v2(n - 2, ans);
            }
        }
    }

    pub fn v3(n: usize) -> usize {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }

        let (mut p1, mut p2) = (0, 1);
        let mut ans = 0;
        for _ in 2..=n {
            ans = p1 + p2;
            p1 = p2;
            p2 = ans;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_fibonacci() {
        assert_eq!(fibonacci::v1(15), 610);

        let mut ans = 0;
        fibonacci::v2(15, &mut ans);
        assert_eq!(fibonacci::v1(15), 610);

        assert_eq!(fibonacci::v3(15), 610);
    }
}
