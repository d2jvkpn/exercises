fn main() {
    let mut data: Vec<u8> = Vec::with_capacity(5); // Vec::new();
    println!("length = {}, capacity = {}", data.len(), data.capacity()); // 0, 5

    data.reserve(2);
    println!("length = {}, capacity = {}", data.len(), data.capacity()); // 0, 5

    data.reserve(12);
    println!("length = {}, capacity = {}", data.len(), data.capacity()); // 0, 12

    let mut tmp = vec![1; 14];
    data.append(&mut tmp);
    println!("length = {}, capacity = {}", data.len(), data.capacity()); // 14, 24
}
