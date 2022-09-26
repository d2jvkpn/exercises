#![allow(dead_code)]

fn main() {
    let mut data = vec![3, 2, 1];

    let k = bubble_sort(&mut data);
    println!("k={}, data={:?}", k, data);

    let mut data = vec![3, 2, 1];

    let k = cock_tail(&mut data);
    println!("k={}, data={:?}", k, data);

    let mut data = vec![3, 1, 2, 5, 4, 6];
    println!("\ndata={:?}", data);
    let len = data.len();
    quick_sort(&mut data, 0, len - 1);
    println!("data={:?}", data);

    let mut data = vec![3, 1, 2, 5, 4, 6];
    println!("\ndata={:?}", data);
    binary_insert_sort(&mut data);
    println!("data={:?}", data);
}

fn bubble_sort(nums: &mut [i32]) -> usize {
    if nums.len() < 2 {
        return 0;
    }
    let (mut k, len) = (0, nums.len());

    for i in 0..len {
        for j in 0..len - 1 - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                k += 1;
            }
        }
    }

    k
}

fn cock_tail(nums: &mut [i32]) -> usize {
    if nums.len() < 2 {
        return 0;
    }
    let (mut k, len) = (0, nums.len());

    for i in 0..len >> 1 {
        for j in i..(len - i - 1) {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                k += 1;
            }
        }

        for j in (i + 1..=(len - i - 1)).rev() {
            if nums[j - 1] > nums[j] {
                nums.swap(j - 1, j);
                k += 1;
            }
        }
    }

    k
}

fn cbic_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len < 2 {
        return;
    }

    for i in 1..len {
        for j in 0..i {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}
/*
    4,3,1,2  @1
    3,4,1,2

    1,4,3,2  @2
    1,3,4,2

    1,3,4,2  @3
    1,2,4,3
    1,2,3,4
*/

fn quick_sort(nums: &mut [i32], low: usize, high: usize) {
    if low >= high {
        return;
    }

    fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
        let (mut lidx, mut ridx) = (low, high);

        loop {
            while lidx <= ridx && nums[lidx] <= nums[low] {
                lidx += 1;
            }
            while lidx <= ridx && nums[low] <= nums[ridx] {
                ridx -= 1;
            }

            println!("low={}, lidx={}, ridx={}, high={}", low, lidx, ridx, high);
            if lidx > ridx {
                println!("~~~ {:?}", nums);
                break;
            } else {
                nums.swap(lidx, ridx);
            }
        }

        nums.swap(low, ridx);

        ridx
    }

    let split = partition(nums, low, high);
    println!("~~~ split={}", split);
    if split > 1 {
        quick_sort(nums, low, split - 1);
    }
    quick_sort(nums, split + 1, high);
}

/*
    3, 1, 2, 5, 4, 6
    1, 3, 2, 5, 4, 6
    1, 2, 3, 5, 4, 6
    1, 2, 3, 4, 5, 6
*/

fn insert_sort(nums: &mut [i32]) {
    for i in 0..nums.len() {
        let mut pos = 1;
        let curr = nums[i];

        while pos > 0 && curr < nums[pos - 1] {
            nums[pos] = nums[pos - 1];
            pos -= 1;
        }
        nums[pos] = curr;
    }
}

fn binary_insert_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let v = nums[i];

        let (mut i1, mut i2) = (0, i - 1);

        while i1 <= i2 {
            let m = (i1 + i2) / 2;

            if nums[m] <= v {
                i1 = m + 1;
            } else {
                if m == 0 {
                    break;
                }
                i2 = m - 1;
            }
        }

        for j in (i1..i).rev() {
            nums.swap(j, j + 1);
        }
    }
}
