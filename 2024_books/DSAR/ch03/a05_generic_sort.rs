fn generic_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for _ in 0..len {
        for j in 0..len - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut numbers = vec![3, 5, 1, 4, 2];
    generic_sort(&mut numbers);
    println!("{:?}", numbers);

    let mut words = vec!["rust", "is", "awesome"];
    generic_sort(&mut words);
    println!("{:?}", words);
}
