use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    let arr = vec![1, 2, 3, 5, 6, 10, 42];

    let ans = binary_search(&arr, 9);
    println!("==> {ans:?}");

    let ans = binary_search(&arr, 5);
    println!("==> {ans:?}");
}

fn binary_search<T: Ord>(arr: &[T], target: T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(&target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }

    None
}
