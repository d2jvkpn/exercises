use std::ops::{Add, AddAssign};

fn main() {
    assert_eq!(sum_of_slice(&[1, 2, 3, 4, 5]), 15);

    let mut sum = 0;
    sum_of_slice2(&[1, 2, 3, 4, 5], &mut sum);
    assert_eq!(sum, 15);

    assert_eq!(tower_of_hanoi(7), 127);
    assert_eq!(tower_of_hanoi(10), 1023);
}

fn sum_of_slice<T: Default + Copy + Add<Output = T>>(nums: &[T]) -> T {
    match nums.len() {
        0 => Default::default(),
        1 => nums[0],
        _ => nums[0] + sum_of_slice(&nums[1..]),
    }
}

fn sum_of_slice2<T: Default + Copy + Add<Output = T> + AddAssign>(nums: &[T], sum: &mut T) {
    match nums.len() {
        0 => {}
        _ => {
            *sum += nums[0];
            sum_of_slice2(&nums[1..], sum);
        }
    }
}

// Tower of Hanoi
fn tower_of_hanoi(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    // initial status: left: -, mid: (1..n), right: -
    // target status:  left: -, mid-,        right: (1..n)

    // status-1; left: (1..n-1), mid: -, right: n
    // move (1..n-1) from mind to left, and then move n from mid to target;
    let steps1 = 1 + tower_of_hanoi(n - 1);

	// steps from status-1 to target status, move (1..n-1) from left to target
    let steps2 = tower_of_hanoi(n - 1);

    return steps1 + steps2;
}
