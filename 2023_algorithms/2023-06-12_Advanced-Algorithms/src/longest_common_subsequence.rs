use std::cmp::max;

// Time: O(2^*(n+m)), Space: O(n+m)
pub fn lc_seq_brute(s1: &[char], s2: &[char]) -> usize {
    help_brute(s1, 0, s2, 0)
}

fn help_brute(s1: &[char], i1: usize, s2: &[char], i2: usize) -> usize {
    if i1 >= s1.len() || i2 >= s2.len() {
        return 0;
    }

    if s1[i1] == s2[i2] {
        return 1 + help_brute(s1, i1 + 1, s2, i2 + 1);
    }

    max(help_brute(s1, i1 + 1, s2, i2), help_brute(s1, i1, s2, i2 + 1))
}

// Time: O(n*m), Space: O(min(n, m))
pub fn lc_seq_dp(s1: &[char], s2: &[char]) -> usize {
    if s1.len() < s2.len() {
        return lc_seq_dp(s2, s1);
    }

    // assert!(s1.len() >= s2.len());
    let (rows, cols) = (s1.len(), s2.len());
    let mut dp = vec![0; cols + 1];
    let mut current = vec![0; cols + 1];

    for i in 0..rows {
        for j in 0..cols {
            if s1[i] == s2[j] {
                current[j + 1] = 1 + dp[j];
            } else {
                current[j + 1] = max(dp[j + 1], current[j]);
            }
        }

        dp.copy_from_slice(&current);
        current.fill_with(|| 0);
    }

    dp[cols]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_vec() {
        let mut v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6];

        v1.copy_from_slice(&v2);
        dbg!(&v1);

        v1.fill_with(|| 0);
        dbg!(&v1);

        v1.clear();
        dbg!(&v1);
    }

    #[test]
    fn t_lc_seq() {
        //
        let s1 = &['A', 'D', 'C', 'B'];
        let s2 = &['A', 'B', 'C'];

        assert_eq!(lc_seq_brute(s1, s2), 2);
        assert_eq!(lc_seq_dp(s1, s2), 2);

        //
        let s1 = &['A', 'D', 'B', 'C'];
        let s2 = &['A', 'B', 'C'];

        assert_eq!(lc_seq_brute(s1, s2), 3);
        assert_eq!(lc_seq_dp(s1, s2), 3);
    }
}
