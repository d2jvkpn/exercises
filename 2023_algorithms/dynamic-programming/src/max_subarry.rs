use std::cmp::max;

fn max_subarray_brute(arr: &[i32]) -> i32 {
    // assert!(arr.len() > 0);
    if arr.is_empty() {
        return 0;
    }

    let mut max_sum = arr[0];

    for i in 0..arr.len() {
        let mut sum_sub = 0;

        for j in i + 1..arr.len() {
            sum_sub += arr[j];
            max_sum = max(max_sum, sum_sub);
        }
    }

    max_sum
}

fn max_subarray_dp(arr: &[i32]) -> i32 {
    // assert!(arr.len() > 0);
    if arr.is_empty() {
        return 0;
    }

    let length = arr.len();
    let mut dp = vec![0; length];

    dp[0] = arr[0];

    for i in 1..length {
        dp[i] = max(dp[i - 1] + arr[i], arr[i]);
    }

    dp.iter().max().unwrap().clone()
}

fn max_subarray_kadane(arr: &[i32]) -> i32 {
    // assert!(arr.len() > 0);
    if arr.is_empty() {
        return 0;
    }

    let (mut max_sum, mut max_sub) = (arr[0], arr[0]);

    for i in 1..arr.len() {
        max_sum = max(max_sum + arr[i], arr[i]);
        max_sub = max(max_sum, max_sub);
    }

    max_sub
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_max_subarray_brute() {
        let arr = vec![4, -1, 2, -7, 3, 4];
        assert_eq!(max_subarray_brute(&arr), 7);
    }

    #[test]
    fn t_max_subarray_dp() {
        let arr = vec![4, -1, 2, -7, 3, 4];
        assert_eq!(max_subarray_dp(&arr), 7);
    }

    #[test]
    fn t_max_subarray_kadane() {
        let arr = vec![4, -1, 2, -7, 3, 4];
        assert_eq!(max_subarray_kadane(&arr), 7);
    }
}
