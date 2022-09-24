#![allow(dead_code)]

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(sequential_search(&nums[..], 3), Some(2));

    assert_eq!(binary_search(&nums[..], 3), Some(2));
    assert_eq!(binary_search(&[1, 2], 3), None);
    assert_eq!(binary_search(&[4, 5], 3), None);
    assert_eq!(binary_search(&[1, 3], 3), Some(1));
    assert_eq!(binary_search(&[3, 5], 3), Some(0));

    assert_eq!(interpolation_search(&nums[..], 3), Some(2));
    assert_eq!(interpolation_search(&[1, 2], 3), None);
    assert_eq!(interpolation_search(&[4, 5], 3), None);
    assert_eq!(interpolation_search(&[1, 3], 3), Some(1));
    assert_eq!(interpolation_search(&[3, 5], 3), Some(0));
}

fn sequential_search(nums: &[i32], num: i32) -> Option<usize> {
    for (i, v) in nums.iter().enumerate() {
        if *v == num {
            return Some(i);
        }
    }

    return None;
}

fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }
    let (mut low, mut high) = (0, nums.len() - 1);

    while low < high {
        let mid = low + (high - low) / 2;
        // println!("low={}, mid={}, high={}", low, mid, high);

        if target == nums[mid] {
            return Some(mid);
        } else if target == nums[low] {
            return Some(low);
        } else if target == nums[high] {
            return Some(high);
        } else if low == mid {
            return None;
        }

        if nums[mid] < target {
            low = mid;
        } else {
            high = mid;
        }
    }

    None
}

fn interpolation_search(nums: &[i32], target: i32) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }
    let (mut low, mut high) = (0, nums.len() - 1);

    while low < high {
        let high_val = nums[high];
        let low_val = nums[low];

        if target < low_val || target > high_val {
            return None;
        }

        let offset = (target - low_val) * (high - low) as i32 / (high_val - low_val);
        let mid = low + offset as usize;
        // println!("low={}, mid={}, high={}", low, mid, high);

        if target == nums[mid] {
            return Some(mid);
        } else if target == nums[low] {
            return Some(low);
        } else if target == nums[high] {
            return Some(high);
        }

        if nums[mid] < target {
            low = mid - 1;
        } else {
            high = mid + 1;
        }
    }

    None
}

fn exponential_search(nums: &[i32], target: i32) -> Option<usize> {
    let size = nums.len();
    if size == 0 {
        return None;
    }

    let mut high = 1;
    while high < size && nums[high] < target {
        high <<= 1;
    }
    let low = high >> 1;

    binary_search(&nums[low..size.min(high + 1)], target)
}
