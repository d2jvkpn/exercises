use rayon;

fn main() {
    let mut ans = vec![127, 9, 99, 10, 42, 12];
    parallel_quick_sort(&mut ans);
    println!("==> ans: {ans:?}");
}

fn parallel_quick_sort<T: Ord + Send + Sync>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // Base case: array is already sorted
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);

    rayon::join(
        || parallel_quick_sort(&mut left[0..pivot_index]),
        || parallel_quick_sort(&mut right[1..]),
    );
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    i
}
