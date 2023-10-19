// Time: O(n*m), Space: O(n*m)
pub fn lc_sub_v1(s1: &str, s2: &str) -> String {
    let (rows, cols) = (s1.len(), s2.len());
    let mut matrix = vec![vec![0; cols + 1]; rows + 1];
    let mut max_length = 0;
    let mut result = String::new();

    for i in 1..=rows {
        for j in 1..=cols {
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                matrix[i][j] = matrix[i - 1][j - 1] + 1;

                if matrix[i][j] > max_length {
                    max_length = matrix[i][j];
                    result = s1[i - max_length..i].to_string();
                }
            }
        }
    }

    result
}

// Time: O(n*m), Space: O(min(n, m))
pub fn lc_sub_v2(s1: &str, s2: &str) -> Option<String> {
    if s1.len() < s2.len() {
        return lc_sub_v2(s2, s1);
    }

    let (rows, cols) = (s1.len(), s2.len());
    let (mut max_length, mut index) = (0, 0);
    let (mut current, mut dp) = (vec![0; cols + 1], vec![0; cols + 1]);

    for i in 1..=rows {
        for j in 1..=cols {
            if s1.chars().nth(i - 1) != s2.chars().nth(j - 1) {
                continue;
            }

            current[j] = dp[j - 1] + 1;

            if current[j] > max_length {
                max_length = current[j];
                index = i - max_length;
            }
        }

        dp.copy_from_slice(&current);
        current.fill_with(|| 0);
    }

    if max_length > 0 {
        Some(s1[index..index + max_length].into())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_lc_sub() {
        //
        let s1 = "abcdxyz";
        let s2 = "xyzabcdki";

        assert_eq!(lc_sub_v1(s1, s2), "abcd".to_string());
        assert_eq!(lc_sub_v2(s1, s2), Some("abcd".to_string()));

        //
        let s1 = "abc";
        let s2 = "defc";

        assert_eq!(lc_sub_v1(s1, s2), "c".to_string());
        assert_eq!(lc_sub_v2(s1, s2), Some("c".to_string()));
    }
}
