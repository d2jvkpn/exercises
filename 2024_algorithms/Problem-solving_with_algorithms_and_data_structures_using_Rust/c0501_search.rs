fn main() {
    println!("Hello, wrold!");
}

fn sequencial_search(nums: &[i32], target: i32) -> Option<usize> {
    for i in 0..nums.len() {
        if nums[i] == target {
            return Some(i);
        }
    }

    return None;
}

fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    if nums.len() == 0 {
        return None;
    }

    let (mut low, mut high) = (0, nums.len() - 1);
    let (mut mid, mut value);

    while low <= high {
        mid = (low + high) >> 1;
        value = nums[mid];

        if value == target {
            return Some(mid);
        }

        if value > target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}

fn binary_search_rec(nums: &[i32], target: i32) -> Option<usize> {
    if nums.len() == 0 {
        return None;
    }

    let (mut low, mut high) = (0, nums.len() - 1);
    let mid = (low + high) >> 1;
    let value = nums[mid];

    if value == target {
        return Some(mid);
    }

    if value > target {
        return binary_search_rec(&nums[..mid], target);
    } else {
        return binary_search_rec(&nums[mid + 1..], target);
    }
}
