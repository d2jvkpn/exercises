fn main() {
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

    let a = [1, 2, 3, 4, 5, 6];
    let a_slice = &a[..3];
    println!("~~~ a_slice: {a_slice:?}");
}

// fn trim_tweet(tweet: &String) -> &str {
fn trim_tweet(tweet: &str) -> &str {
    &tweet[..20]
}

fn get_chars(tweet: &str, size: usize) -> &str {
    let len = tweet.chars().take(size).map(|v| v.len_utf8()).sum();
    &tweet[..len]
}
