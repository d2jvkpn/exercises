fn contains_duplicate(nums: &[i64]) -> bool {
    if nums.len() < 2 {
        return false;
    }

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] {
                return true;
            }
        }
    }

    false
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 2];
    println!(">>> contains_duplicate: {}", contains_duplicate(&arr));
}
