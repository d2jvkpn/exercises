fn main() {
    // println!("Hello, wrold!");
}

fn majority_element(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    nums.iter().for_each(|v| {
        map.entry(v).and_modify(|v| *v += 1).or_insert(1);
    });

    for (k, v) in map {
        if v > nums.len() / 2 {
            return *k;
        }
    }
    
    0
}

/*
169. Majority Element

Given an array nums of size n, return the majority element.

The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.



Example 1:

Input: nums = [3,2,3]
Output: 3
Example 2:

Input: nums = [2,2,1,1,1,2,2]
Output: 2


Constraints:

n == nums.length
1 <= n <= 5 * 104
-109 <= nums[i] <= 109


Follow-up: Could you solve the problem in linear time and in O(1) space?
*/
