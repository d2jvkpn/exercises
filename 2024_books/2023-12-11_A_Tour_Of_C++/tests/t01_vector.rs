fn main() {
    let mut v1: Vec<i32> = Vec::with_capacity(5);

    v1.push(42);

    println!("v1[1]: {}", v1[1]); // EROOR: index out of bounds: the len is 1 but the index is 1
}
