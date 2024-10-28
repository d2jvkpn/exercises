use rayon;

pub fn merge_sort<T: Ord + Send + Sync + Clone>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return; // Base case: array of length 0 or 1 is already sorted
    }

    let mid = slice.len() / 2;
    let len = slice.len();
    let (left, right) = slice.split_at_mut(mid);

    // Sort the halves in parallel
    rayon::join(|| merge_sort(left), || merge_sort(right));

    let mut sorted = Vec::with_capacity(len);
    merge(left, right, &mut sorted);

    // Copy sorted elements back to original array
    for (i, elem) in sorted.into_iter().enumerate() {
        slice[i] = elem;
    }
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], merged: &mut Vec<T>) {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    let mut left_val = left_iter.next();
    let mut right_val = right_iter.next();

    while left_val.is_some() && right_val.is_some() {
        if left_val < right_val {
            merged.push(left_val.take().unwrap().clone());
            left_val = left_iter.next();
        } else {
            merged.push(right_val.take().unwrap().clone());
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

pub fn merge_sort_v2<T: Ord + Send + Sync>(slice: &mut [T]) {
    let len = slice.len();
    if len < 2 {
        return; // Base case: arrays of length 0 or 1 are already sorted
    }

    let mid = len / 2;
    let (left, right) = slice.split_at_mut(mid);

    rayon::join(|| merge_sort_v2(left), || merge_sort_v2(right));

    merge_v2(slice, mid);
}

fn merge_v2<T: Ord + Send + Sync>(slice: &mut [T], mut index: usize) {
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
