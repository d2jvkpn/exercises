// generate a sorted slice from the end(S), by moving the lastest value to the head of S
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

// generate a sorted slice from the head(S), by moving the next element to the right position of S
pub fn insertion_sort<T: Ord>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let mut j = i;

        while j > 0 && slice[j - 1] > slice[j] {
            slice.swap(j - 1, j);
            j -= 1;
        }
    }
}

// get the index of min element in the slice and move to then front
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
