use rayon;

fn main() {
    let mut ans = vec![127, 9, 99, 10, 42, 12];
    parallel_merge_sort(&mut ans);
    println!("==> ans: {ans:?}");
}

fn parallel_merge_sort<T: Ord + Send + Sync + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return; // Base case: arrays of length 0 or 1 are already sorted
    }

    let mid = len / 2;
    let (left, right) = arr.split_at_mut(mid);
    
    rayon::join(
        || parallel_merge_sort(left),
        || parallel_merge_sort(right),
    );

    merge(arr, mid);
}

fn merge<T: Ord + Send + Sync + Clone>(arr: &mut [T], mut mid: usize) {
    let mut k = 0;

    while k < mid {
        // dbg!(&[k, mid]);
        if arr[k] <= arr[mid] {
            k += 1;
        } else {
            arr.swap(k, mid);
            if mid < arr.len() - 1 {
                mid += 1;
            }
        }
    }

    return;
}
