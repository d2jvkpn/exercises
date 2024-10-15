use rayon;

fn main() {
    let mut ans = vec![127, 9, 99, 10, 42, 12];
    parallel_merge_sort(&mut ans);
    println!("==> ans: {ans:?}");
}

fn parallel_merge_sort<T: Ord + Send + Sync>(slice: &mut [T]) {
    let len = slice.len();
    if len < 2 {
        return; // Base case: arrays of length 0 or 1 are already sorted
    }

    let mid = len / 2;
    let (left, right) = slice.split_at_mut(mid);
    
    rayon::join(
        || parallel_merge_sort(left),
        || parallel_merge_sort(right),
    );

    merge(slice, mid);
}

fn merge<T: Ord + Send + Sync>(slice: &mut [T], mut index: usize) {
    let mut k = 0;

    while k < index {
        // dbg!(&[k, mid]);
        if slice[k] <= slice[index] {
            k += 1;
        } else {
            slice.swap(k, index);
            if index < slice.len() - 1 {
                index += 1;
            }
        }
    }
}
