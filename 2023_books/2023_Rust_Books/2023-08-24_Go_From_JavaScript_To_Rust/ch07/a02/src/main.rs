fn main() {
    println!("TESTING:1234567890");

    let _my_real_string = "string literal!".to_owned();
    // String::from_utf8_unchecked(self.as_bytes().to_owned());
    // .to_string(), .into(), String::from(), format!()

    needs_a_string("string literal!".to_string());
}

fn needs_a_string(_argument: String) {}
