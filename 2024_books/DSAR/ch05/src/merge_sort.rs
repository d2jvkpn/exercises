pub fn merge_sort<T: Ord + Clone>(slice: &[T]) -> Vec<T> {
    let len = match slice.len() {
        // Base case: arrays of length 0 or 1 are already sorted
        v if v < 2 => return slice.to_vec(),
        v => v,
    };

    let mut left_sorted = merge_sort(&slice[0..len / 2]);
    let mut right_sorted = merge_sort(&slice[len / 2..]);

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
