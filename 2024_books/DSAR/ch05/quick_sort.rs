fn main() {
    let mut ans = vec![127, 9, 99, 10, 42, 12];
    quick_sort(&mut ans);
    println!("==> ans: {ans:?}");
}

fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // Base case: array is already sorted
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    quick_sort(&mut left[..pivot_index]);
    quick_sort(&mut right[1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    let mut index = 0;

    for i in 0..len - 1 {
        if arr[i] <= arr[len - 1] {
            arr.swap(index, i);
            index += 1;
        }
    }

    arr.swap(index, len - 1);
    index
}
