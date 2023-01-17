fn main() {
    // This is a comment

    /* This is a
    multi line
    comment
    */

    // learning some basic output commands
    println!("Hello, world!");

    println!("The value of the constant is {}", 42);
    println!(
        "{language} is a system programming language which is coold to {activity} in",
        language = "Rust",
        activity = "code"
    );

    println!(
        "This is going to be
printed on multiple
line"
    );

    println!(r#"asas " ' \"#);

    let a = "Hello";
    println!("{a}...");
}
