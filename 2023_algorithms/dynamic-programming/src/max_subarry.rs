use std::cmp::max;

fn max_subarray_brute(arr: &[i32]) -> i32 {
    // assert!(arr.len() > 0);
    if arr.is_empty() {
        return 0;
    }

    let mut max_sum = arr[0];

    for i in 0..arr.len() {
        for j in i..arr.len() {
            let v = arr[i..j].iter().sum();
            if v > max_sum {
                max_sum = v;
            }
        }
    }

    max_sum
}

fn max_subarray_kadane(arr: &[i32]) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn t_max_subarray_brute() {
        let arr = vec![4, -1, 2, -7, 3, 4];
        assert_eq!(max_subarray_brute(&arr), 7);
    }

    fn t_max_subarray_kadane() {
        let arr = vec![4, -1, 2, -7, 3, 4];
        assert_eq!(max_subarray_kadane(&arr), 7);
    }
}
