pub fn merge_sort<T: Ord + Clone>(slice: &mut [T]) {
    let len = slice.len();
    if len < 2 {
        return; // Base case: arrays of length 0 or 1 are already sorted
    }

    let mid = len / 2;

    merge_sort(&mut slice[0..mid]);
    merge_sort(&mut slice[mid..]);

    merge(slice, mid);
}

fn merge<T: Ord + Clone>(slice: &mut [T], mut mid: usize) {
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
