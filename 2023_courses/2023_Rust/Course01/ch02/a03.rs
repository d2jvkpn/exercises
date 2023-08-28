fn main() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("length: {}, capacity: {}", nums.len(), nums.capacity());

    nums.push(6);
    nums.remove(1);
    println!("nums: {:?}, nums[100]: {:?}", nums, nums.get(100));

    println!("nums contains 3: {}", nums.contains(&3));

    println!("nums[1..=2]: {:?}", &nums[1..=2]);
}
