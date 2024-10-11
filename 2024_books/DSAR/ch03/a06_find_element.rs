// fn find_element<T: PartialEq>(arr: &[T], target: T) -> Result<usize, &'static str> {
fn find_element<T: PartialEq>(arr: &[T], target: T) -> Result<usize, &str> {
    for (index, element) in arr.iter().enumerate() {
        if *element == target {
            return Ok(index);
        }
    }
    Err("Element not found")
}

fn main() {
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
