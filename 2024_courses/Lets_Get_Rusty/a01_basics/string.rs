#![allow(unused_variables)]

fn main() {
    //
    let tweet = String::from("This is my tweet and it's very very long.");

    let trimed_tweet = trim_tweet(&tweet); // pass &String, derefrence conversion
    println!("~~~ trimed tweet: {trimed_tweet}");

    let hello_str = "你好，世界";
    let hello_bts = hello_str.as_bytes();

    println!(
        "~~~ len(hello_str): {}\n    len(hello_bts): {}\n    hello_str.chars().count(): {}",
        hello_str.len(),
        hello_bts.len(),
        hello_str.chars().count(),
    );

    println!("~~~ hello_bts[..3]: {:?}", &hello_bts[..3]);

    println!("~~~ get_chars(hello_str, 3): {:?}", get_chars(hello_str, 3));
    println!(r#"~~~ get_chars("你好"， 20): {:?}"#, get_chars("你好", 24));

    //
    let a = [1, 2, 3, 4, 5, 6];
    let a_slice = &a[..3];
    println!("~~~ a_slice: {a_slice:?}");

    //
    let s1: &str = "Hello, world!";
    let s2: String = String::from("Hello, world!");
    let s3: String = "Hello, world!".to_string();
    let s4: String = "Hello, world!".to_owned();
    let s5: &str = &s4[..];
    let s6: String = "Hello, world!".into();

    //
    let mut s: String = String::from("foo");
    s.push_str(" bar");
    dbg!(&s);
    s.replace_range(.., "baz");
    dbg!(&s);

    //
    let s1 = String::from("Hello");
    let s2 = String::from("world");
    let s3 = s1 + ", " + &s2; // s1 was removed
    dbg!((&s2, &s3));

    let s1 = ["frist", "second"].concat();
    let s2 = format!("{} {}", "first", "second");
    let s3 = concat!("first", ", ", "second");
    dbg!((&s1, &s2, &s3));

    //
    let s1: &str = "你a好，Rust 编程语言！";
    println!(
        "~~~ len(s1): {}, s1.bytes().count(): {}, s1.chars().count(): {}",
        s1.len(),
        s1.bytes().count(),
        s1.chars().count()
    );
    println!("~~~ s1[0..3]: {}, s1[3..4]: {}", &s1[0..3], &s1[3..4]);

    println!("~~~ bytes:");
    for b in s1.bytes() {
        print!("{b}, ")
    }
    println!("");

    println!("~~~ chars:");
    for c in s1.chars() {
        print!("'{c}', ")
    }
    println!("")
}

// fn trim_tweet(tweet: &String) -> &str {
fn trim_tweet(tweet: &str) -> &str {
    &tweet[..20]
}

fn get_chars(tweet: &str, size: usize) -> &str {
    let len = tweet.chars().take(size).map(|v| v.len_utf8()).sum();
    &tweet[..len]
}
