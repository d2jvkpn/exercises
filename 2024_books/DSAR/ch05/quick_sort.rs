fn main() {
    let mut ans = vec![127, 9, 99, 10, 42, 12];
    quick_sort(&mut ans);
    println!("==> ans: {ans:?}");
}

fn quick_sort<T: Ord>(slice: &mut [T]) {
    if slice.len() < 2 {
        return; // Base case: array is already sorted
    }

    let pivot_index = partition(slice);
    let (left, right) = slice.split_at_mut(pivot_index);

    quick_sort(&mut left[..pivot_index]);
    quick_sort(&mut right[1..]);
}

fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let len = slice.len();
    let pivot_index = len / 2;

    slice.swap(pivot_index, len - 1);
    let mut index = 0;

    for i in 0..len - 1 {
        if slice[i] <= slice[len - 1] {
            slice.swap(index, i);
            index += 1;
        }
    }

    slice.swap(index, len - 1);
    index
}
