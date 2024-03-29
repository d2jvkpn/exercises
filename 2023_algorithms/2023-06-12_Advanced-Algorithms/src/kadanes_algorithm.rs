use std::cmp::max;

// brute force, O(n**2), O(n)
fn max_subarray_brute(arr: &[i32]) -> i32 {
    // assert!(arr.len() > 0);
    if arr.is_empty() {
        return 0;
    }

    let mut max_sum = arr[0];

    for i in 0..arr.len() {
        let mut curent = 0;

        // for j in i + 1..arr.len() {
        //     curent += arr[j];
        //     max_sum = max(max_sum, curent);
        // }
        for v in arr.iter().skip(i + 1) {
            curent += v;
            max_sum = max(max_sum, curent);
        }
    }

    max_sum
}

// dynamic programming, O(n), O(n)
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

    // dp.iter().max().unwrap().clone()
    *dp.iter().max().unwrap()
}

// kafane's algorithm, O(n), 1
fn max_subarray_kadane(arr: &[i32]) -> i32 {
    // assert!(arr.len() > 0);
    if arr.is_empty() {
        return 0;
    }

    let (mut max_sum, mut current) = (arr[0], arr[0]);

    // for i in 1..arr.len() {
    //     current = max(current + arr[i], arr[i]);
    //     max_sum = max(max_sum, current);
    // }
    for v in arr.iter().skip(1) {
        current = max(current + *v, *v);
        max_sum = max(max_sum, current);
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    static ARR1: [i32; 6] = [4, -1, 2, -7, 3, 4];
    static EXP1: i32 = 7;

    #[test]
    fn t_max_subarray() {
        assert_eq!(max_subarray_brute(&ARR1), EXP1);
        assert_eq!(max_subarray_dp(&ARR1), EXP1);
        assert_eq!(max_subarray_kadane(&ARR1), EXP1);
    }
}
