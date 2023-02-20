mod factorial {
    pub fn v1(n: usize) -> usize {
        println!("--> factorial::v1 {}", n);
        if n <= 1 {
            return 1;
        }

        let ans = n * v1(n - 1);
        println!("<-- {}, got {}", n, ans);
        return ans;
    }

    pub fn v2(n: usize, ans: &mut usize) {
        if n <= 1 {
            return;
        }

        *ans *= n;
        println!("--> facorial::v2 {}, {}", n, ans);
        v2(n - 1, ans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_factorial() {
        assert_eq!(factorial::v1(5), 120);

        let mut ans = 1;
        factorial::v2(5, &mut ans);
        assert_eq!(ans, 120);
    }
}
