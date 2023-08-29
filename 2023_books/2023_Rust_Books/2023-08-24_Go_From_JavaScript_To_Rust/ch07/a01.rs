fn main() {
    print_type_of(&"Hello, wrold!");
    print_type_of(&String::new());
}

fn print_type_of<T>(_: &T) {
    println!("Type is: {}", std::any::type_name::<T>());
}
