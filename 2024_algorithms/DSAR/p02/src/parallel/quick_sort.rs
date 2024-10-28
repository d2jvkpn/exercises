use rayon;

pub fn quick_sort<T: Ord + Send + Sync>(slice: &mut [T]) {
    if slice.len() < 2 {
        return; // Base case: array is already sorted
    }

    let pivot_index = partition(slice);
    let (left, right) = slice.split_at_mut(pivot_index);

    rayon::join(|| quick_sort(&mut left[0..pivot_index]), || quick_sort(&mut right[1..]));
}

fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let len = slice.len();
    let pivot_index = len / 2;
    let mut i = 0;

    slice.swap(pivot_index, len - 1);

    for j in 0..len - 1 {
        if slice[j] <= slice[len - 1] {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, len - 1);
    i
}
