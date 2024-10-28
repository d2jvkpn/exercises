fn main() {
    // 1.
    let mut numbers = vec![3, 5, 1, 4, 2];
    generic_sort(&mut numbers);
    println!("{:?}", numbers);

    let mut words = vec!["rust", "is", "awesome"];
    generic_sort(&mut words);
    println!("{:?}", words);

    // 2.
    let numbers = vec![10, 20, 30, 40, 50];

    match find_element(&numbers, 30) {
        Ok(index) => println!("Element found at index: {}", index),
        Err(e) => println!("Error: {}", e),
    }

    match find_element(&numbers, 60) {
        Ok(index) => println!("Element found at index: {}", index),
        Err(e) => println!("Error: {}", e),
    }
}

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

// fn find_element<T: PartialEq>(arr: &[T], target: T) -> Result<usize, &'static str> {
fn find_element<T: PartialEq>(arr: &[T], target: T) -> Result<usize, &str> {
    for (index, element) in arr.iter().enumerate() {
        if *element == target {
            return Ok(index);
        }
    }
    Err("Element not found")
}
