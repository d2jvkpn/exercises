// Option is pre-imported by Rust's std::prelude

fn main() ->  {
    //println!("Hello, world!");
    println!("Some: {:?}, None: {:?}", return_some(), return_none());

    println!("OK: {:?}, Err: {:?}", return_ok(), return_err());

    if let Err(e) = return_err() {
        println!("~~~ {}", e.0);
    }

    let ans = return_err().unwrap_or("default".to_string());
    println!("~~ ans: {}", ans);

    let ans = return_err().unwrap_or_else(|_| "default".to_string());
    println!("~~ ans: {}", ans);

    assert_eq!(return_err().ok(), None);
    assert!(return_err().unwrap_or_default().is_empty());

    let err = MyError("something is wrong".to_string());
    assert!(return_some().ok_or(err).is_err());
}

fn return_some() -> Option<String> {
    Some("My String".to_owned())
}

fn return_none() -> Option<String> {
    None
}

#[derive(Debug)]
struct MyError(String);

fn return_ok() -> Result<String, MyError> {
    Ok("This turned out great!".to_owned())
}

fn return_err() -> Result<String, MyError> {
    Err(MyError("This failed horribly.".to_owned()))
}
