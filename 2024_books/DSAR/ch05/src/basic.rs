pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        // Assume the minimum element is at the current position
        let mut min_index = i;

        // Find the index of the smallest element in the unsorted portion
        for j in (i + 1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        // Swap the found minimum element with the first unsorted element
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    let mut swapped: bool;

    for i in 0..len {
        swapped = false;
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
