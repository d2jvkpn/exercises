fn main() {
    let hello = "Hello, wrold!".to_string();
    let chars: Vec<char> = hello.chars().collect();
    println!("{:?}", chars);
}
