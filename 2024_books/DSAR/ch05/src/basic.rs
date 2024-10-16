pub fn bubble_sort<T: Ord>(slice: &mut [T]) {
    let len = slice.len();
    let mut swapped: bool;

    for i in 0..len {
        swapped = false;

        for j in 0..(len - 1 - i) {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

pub fn insertion_sort<T: Ord>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let mut j = i;

        while j > 0 && slice[j - 1] > slice[j] {
            slice.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn selection_sort<T: Ord>(slice: &mut [T]) {
    let len = slice.len();

    for i in 0..len {
        // Assume the minimum element is at the current position
        let mut index = i;

        // Find the index of the smallest element in the unsorted portion
        for j in (i + 1)..len {
            if slice[j] < slice[index] {
                index = j;
            }
        }

        // Swap the found minimum element with the first unsorted element
        if index != i {
            slice.swap(i, index);
        }
    }
}
