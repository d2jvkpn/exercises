use std::{
    cmp::{max, min},
    collections::HashSet,
};

fn contains_duplicates_brute(nums: &[i32], k: usize) -> bool {
    let length = nums.len();
    if k < 1 || k > length {
        return false;
    }

    for i in 0..(length - k) {
        for j in (i + 1)..(i + k) {
            if nums[i] == nums[j] {
                return true;
            }
        }
    }

    false
}

fn contains_duplicates_set(nums: &[i32], k: usize) -> bool {
    let length = nums.len();
    if k < 1 || k > length {
        return false;
    }

    let mut low = 0;
    let mut set = HashSet::new();

    for (high, v) in nums.iter().enumerate() {
        if high - low + 1 > k {
            set.remove(v);
            low += 1;
        }

        if set.contains(&v) {
            return true;
        }

        set.insert(v);
    }

    false
}

fn longest_repeat(nums: &[i32]) -> usize {
    let (mut low, mut max_len) = (0, 0);

    for high in 0..nums.len() {
        if nums[low] != nums[high] {
            low = high;
            continue;
        }
        max_len = max(max_len, high - low + 1)
    }

    max_len
}

// minimum length subarray where the sum is greater than or equal to the target
fn min_len_sum_ge(nums: &[usize], target: usize) -> usize {
    let (mut low, mut min_len) = (0, nums.len() + 1);
    let mut sum = 0;

    for high in 0..nums.len() {
        sum += nums[high];
        if sum >= target {
            min_len = min(min_len, high - low + 1);
            sum -= nums[low];
            low += 1;
        }
    }

    if min_len == nums.len() + 1 {
        0
    } else {
        min_len
    }
}
