fn main() {
    let ans = vec![127, 9, 99, 10, 42, 12];
    let ans = merge_sort(&ans);
    println!("==> ans: {ans:?}");

    let mut ans = vec![127, 9, 99, 10, 42, 12];
    merge_sort_inplace(&mut ans);
    println!("==> ans: {ans:?}");
}

fn merge_sort<T: Ord + Clone>(slice: &[T]) -> Vec<T> {
    let len = slice.len();
    if len < 2 {
        return slice.to_vec(); // Base case: arrays of length 0 or 1 are already sorted
    }

    let mid = len / 2;
    let left = &slice[0..mid];
    let right = &slice[mid..];

    let mut left_sorted = merge_sort(left);
    let mut right_sorted = merge_sort(right);

    let mut merged = Vec::with_capacity(len);
    merge(&mut left_sorted, &mut right_sorted, &mut merged);

    merged
}

fn merge<T: Ord + Clone>(left: &mut Vec<T>, right: &mut Vec<T>, merged: &mut Vec<T>) {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    let mut left_val = left_iter.next();
    let mut right_val = right_iter.next();

    while left_val.is_some() && right_val.is_some() {
        if left_val < right_val {
            merged.push(left_val.cloned().unwrap());
            left_val = left_iter.next();
        } else {
            merged.push(right_val.cloned().unwrap());
            right_val = right_iter.next();
        }
    }

    while let Some(val) = left_val {
        merged.push(val.clone());
        left_val = left_iter.next();
    }

    while let Some(val) = right_val {
        merged.push(val.clone());
        right_val = right_iter.next();
    }
}

fn merge_sort_inplace<T: Ord + Clone>(slice: &mut [T]) {
    let len = slice.len();
    if len < 2 {
        return; // Base case: arrays of length 0 or 1 are already sorted
    }

    let mid = len / 2;

    merge_sort_inplace(&mut slice[0..mid]);
    merge_sort_inplace(&mut slice[mid..]);

    merge_inplace(slice, mid);
}

fn merge_inplace<T: Ord + Clone>(slice: &mut [T], mut mid: usize) {
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
