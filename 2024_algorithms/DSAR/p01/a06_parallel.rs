use std::thread;

fn merge_sort_parallel<T: Ord + Clone + Send + 'static>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let (left, right) = arr.split_at_mut(mid);

    // Using `to_vec()` here is necessary for cloning because threads may outlive `arr`.
    let left_handle = {
        let left = left.to_vec();
        thread::spawn(move || {
            let mut left = left;
            merge_sort_parallel(&mut left);
            left
        })
    };

    let right_handle = {
        let right = right.to_vec();
        thread::spawn(move || {
            let mut right = right;
            merge_sort_parallel(&mut right);
            right
        })
    };

    let left_sorted = left_handle.join().unwrap();
    let right_sorted = right_handle.join().unwrap();

    merge(arr, &left_sorted, &right_sorted);
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    merge_sort_parallel(&mut arr);
    println!("Sorted array: {:?}", arr);
}
