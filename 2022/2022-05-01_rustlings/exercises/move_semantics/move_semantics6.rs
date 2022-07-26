// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references

fn main() {
    let data = "rust is great!".to_string();

    println!("last char: {:?}", get_char(&data));

    string_uppercase(&data);

    println!("data = {}", data);
}

// Should not take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &str) {
    let data = data.to_uppercase();

    println!("string_uppercase: {}", data);
}
