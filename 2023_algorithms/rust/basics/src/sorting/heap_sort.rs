pub fn min_heap_sort<T: Ord>(arr: &mut [T]) {
    min_heap(arr);

    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        heapify(&mut arr[..i], 0, |a, b| a < b);
    }
}

pub fn max_heap_sort<T: Ord>(arr: &mut [T]) {
    max_heap(arr);

    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        heapify(&mut arr[..i], 0, |a, b| a > b);
    }
}

pub fn min_heap<T: Ord>(arr: &mut [T]) {
    for i in (0..arr.len() / 2).rev() {
        heapify(arr, i, |a, b| a < b);
    }
}

pub fn max_heap<T: Ord>(arr: &mut [T]) {
    for i in (0..arr.len() / 2).rev() {
        heapify(arr, i, |a, b| a > b);
    }
}

fn heapify<T: Ord>(arr: &mut [T], idx: usize, comparator: fn(&T, &T) -> bool) {
    let len = arr.len();
    let mut target = idx;
    let (left, right) = (2 * idx + 1, 2 * idx + 2);

    if left < len && !comparator(&arr[left], &arr[target]) {
        target = left;
    }

    if right < len && !comparator(&arr[right], &arr[target]) {
        target = right;
    }

    if target != idx {
        arr.swap(idx, target);
        heapify(arr, target, comparator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_heap_sort() {
        let mut nums = vec![9, 7, 5, 11, 12, 2, 14, 3, 10, 6];
        println!("Before: {:?}", nums);

        min_heap_sort(&mut nums);
        println!("After: {:?}", nums);
        assert_eq!(nums, vec![2, 3, 5, 6, 7, 9, 10, 11, 12, 14]);

        max_heap_sort(&mut nums);
        println!("After: {:?}", nums);
        assert_eq!(nums, vec![14, 12, 11, 10, 9, 7, 6, 5, 3, 2]);
    }
}
