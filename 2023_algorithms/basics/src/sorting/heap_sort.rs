// author: ChatGPT
pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    // Build max-heap
    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }

    // Extract elements from the max-heap and sort the array
    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify<T: Ord>(arr: &mut [T], len: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < len && arr[left] > arr[largest] {
        largest = left;
    }

    if right < len && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, len, largest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_heap_sort() {
        let mut nums = vec![9, 7, 5, 11, 12, 2, 14, 3, 10, 6];
        println!("Before: {:?}", nums);
        heap_sort(&mut nums);
        println!("After: {:?}", nums);
        assert_eq!(nums, vec![2, 3, 5, 6, 7, 9, 10, 11, 12, 14]);
    }
}
