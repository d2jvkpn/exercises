#![allow(dead_code)]

const NUMS: [i32; 10] = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];

fn main() {
    let mut nums = vec![2, 1];
    quick_sort(&mut nums);
    dbg!(&nums);
}

fn bubble_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len < 2 {
        return;
    }

    let mut swapped;

    for i in 0..len {
        swapped = false;

        for j in i..(len - 1) {
            // curren max value nums[j]
            if nums[j] < nums[j + 1] {
                nums.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

// can't believe it can sort
fn cbics(nums: &mut [i32]) {
    let len = nums.len();
    if len < 2 {
        return;
    }

    for i in 1..len {
        for j in 0..i {
            if nums[j] > nums[i] {
                nums.swap(i, j);
            }
        }
    }
}

pub fn quick_sort<T: Copy + Ord>(arr: &mut [T]) {
    fn partition<T: Copy + Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
        let mut pivot = low;
        let value = arr[high];

        for i in low..high {
            if arr[i] <= value {
                if pivot != i {
                    arr.swap(pivot, i);
                }
                pivot += 1;
            }
        }

        arr.swap(pivot, high);
        pivot
    }

    fn qs<T: Copy + Ord>(arr: &mut [T], low: usize, high: usize) {
        if low >= high {
            return;
        }

        let pivot = partition(arr, low, high);

        if low + 1 < pivot {
            qs(arr, low, pivot - 1);
        }

        if pivot + 1 < high {
            qs(arr, pivot + 1, high);
        }
    }

    if arr.len() < 2 {
        return;
    }

    qs(arr, 0, arr.len() - 1);
}
