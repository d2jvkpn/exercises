fn main() {
    let another_str = "case";
    let test_closure = move |string_input| {
        println!("{} {}", string_input, another_str);
    };
    // another_str was moved

    test_closure("test");
}
