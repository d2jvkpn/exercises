fn main() {
    let str1 = "Hello";
    println!("str1: {:?}", str1);

    let mut str2 = String::from("你-ABC");
    println!("str2: {:?}", str2);

    str2.push('D');
    println!("str2: {:?}", str2);
    str2.pop();
    println!("str2: {:?}", str2);

    str2.push_str(" - D");
    println!("str2: {:?}", str2);

    println!(
        "--- is_empty: {}, length: {}, contains \"BC\": {}, capacity:{}, bytes: {}, chars: {}",
        str2.is_empty(),
        str2.len(),
        str2.contains("BC"),
        str2.capacity(),
        str2.bytes().len(),
        str2.chars().collect::<Vec<_>>().len(),
    );

    let mut str3 = String::from("你好");
    println!("--- {:?}, length: {}, bytes: {}", str3, str3.len(), str3.bytes().len());
    str3.pop();

    println!(
        "--- {:?}, length: {}, bytes: {}, chars: {}",
        str3,
        str3.len(),
        str3.bytes().len(),
        str3.chars().collect::<Vec<_>>().len(),
    );

    // str3.trim() => &str

    let val: i32 = "42".parse().unwrap();
    println!(
        "{}, {}, {}, {}, {}",
        42.to_string(),
        'A'.to_string(),
        "ABC".to_string(),
        val,
        format!("{}-{}", "Evol", "Mason")
    );

    let str4 = "OOO".to_string();
    println!("{}", str4 + "111"); // str4 is moved
}
