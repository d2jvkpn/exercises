fn is_palindrome(nums: &[i32]) -> bool {
    if nums.is_empty() {
        return false;
    }

    let (mut low, mut high) = (0, nums.len() - 1);

    while low < high {
        if nums[low] != nums[high] {
            return false;
        }
        (low, high) = (low + 1, high - 1);
    }

    true
}

// given a sorted input array
fn sum_up_eq_target(nums: &[i32], target: i32) -> Option<[usize; 2]> {
    let (mut low, mut high) = (0, nums.len() - 1);
    let mut sum;

    while low < high {
        sum = nums[low] + nums[high];

        if sum == target {
            return Some([low, high]);
        }

        if sum < target {
            low += 1;
        } else {
            high -= 1;
        }
    }

    None
}
