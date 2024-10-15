fn main() {
    let mut ans = vec![127, 9, 99, 10, 42, 12];
    merge_sort(&mut ans);
    println!("==> ans: {ans:?}");
}

fn merge_sort<T: Ord + Send + Sync + Clone>(slice: &mut [T]) {
    let len = slice.len();
    if len < 2 {
        return; // Base case: arrays of length 0 or 1 are already sorted
    }

    let mid = len / 2;
    let (left, right) = slice.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);

    merge(slice, mid);
}

fn merge<T: Ord + Send + Sync + Clone>(slice: &mut [T], mut mid: usize) {
    let mut k = 0;

    while k < mid {
        // dbg!(&[k, mid]);
        if slice[k] <= slice[mid] {
            k += 1;
        } else {
            slice.swap(k, mid);
            if mid < slice.len() - 1 {
                mid += 1;
            }
        }
    }

    return;
}
