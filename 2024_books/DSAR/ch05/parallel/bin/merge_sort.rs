use rayon;

fn main() {
    let mut ans = vec![127, 9, 99, 10, 42, 12];
    merge_sort(&mut ans);
    println!("==> ans: {ans:?}");
}

fn merge_sort<T: Ord + Send + Sync + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // Base case: array of length 0 or 1 is already sorted
    }

    let mid = arr.len() / 2;
    let len = arr.len();
    let (left, right) = arr.split_at_mut(mid);

    // Sort the halves in parallel
    rayon::join(
        || merge_sort(left),
        || merge_sort(right),
    );

    let mut sorted = Vec::with_capacity(len);
    merge(left, right, &mut sorted);

    // Copy sorted elements back to original array
    for (i, elem) in sorted.into_iter().enumerate() {
        arr[i] = elem;
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
