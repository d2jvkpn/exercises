fn merge_sort<T: Copy + Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    merge(arr, mid);
}

/*
// author: ChatGPT
fn merge(arr: &mut [i32], mid: usize) {
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut arr_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            arr[arr_idx] = left[left_idx];
            left_idx += 1;
        } else {
            arr[arr_idx] = right[right_idx];
            right_idx += 1;
        }
        arr_idx += 1;
    }

    while left_idx < left.len() {
        arr[arr_idx] = left[left_idx];
        left_idx += 1;
        arr_idx += 1;
    }

    while right_idx < right.len() {
        arr[arr_idx] = right[right_idx];
        right_idx += 1;
        arr_idx += 1;
    }
}
*/

fn merge<T: Copy + Ord>(arr: &mut [T], mid: usize) {
    if mid == 0 {
        return;
    }

    for idx in mid..arr.len() {
        let mut c = idx;
        while c > 0 && arr[c] < arr[c - 1] {
            arr.swap(c, c - 1);
            c -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_merge_sort() {
        let mut arr = vec![9, 4, 6, 2, 7, 1, 8, 3, 5];
        println!("~~~ Before: {:?}", arr);
        merge_sort(&mut arr);
        println!("~~~ After: {:?}", arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
