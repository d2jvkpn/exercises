fn main() {
    // println!("Hello, wrold!");

    let vec = vec![2, 3, 1, 1, 4];
    println!("~~~ {}", can_jump(vec));
}

fn can_jump(nums: Vec<i32>) -> bool {
    let mut next = 0;

    let mut visited = Vec::with_capacity(nums.len());
    (0..nums.len()).for_each(|_| visited.push(false));

    while next < nums.len() && !visited[next] {
        // dbg!(&next);
        visited[next] = true;
        next += nums[next] as usize;

        if next >= nums.len() - 1 {
            return true;
        }
    }

    false
}

/*
55. Jump Game

You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.

Return true if you can reach the last index, or false otherwise.



Example 1:

Input: nums = [2,3,1,1,4]
Output: true
Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
Example 2:

Input: nums = [3,2,1,0,4]
Output: false
Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.


Constraints:

1 <= nums.length <= 104
0 <= nums[i] <= 105
*/
