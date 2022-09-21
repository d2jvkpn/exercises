fn main() {
    let mut result: Vec<u32>;

    result = Vec::new();
    println!("{:?}, {}", result, result.len());

    push(&mut result);
    println!("{:?}, {}", result, result.len());

    let cashes = [1, 5, 10, 20, 50];
    for i in 0..cashes.len() {
        println!("{}", cashes[i]);
        println!("~~~ {:?}", &cashes[i..]);
    }
}

fn push(result: &mut Vec<u32>) {
    result.push(100);
}
