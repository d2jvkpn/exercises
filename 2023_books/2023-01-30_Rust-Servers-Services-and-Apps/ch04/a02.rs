use std::io::Error;

type MyResult<T> = Result<T, Error>;

fn main() {
    // println!("Hello, wrold!");

    let r1: MyResult<i32> = Ok(42);
    let r2: MyResult<i32> = MyResult::Ok(42);

    println!("r1={:?}, r2={:?}", r1, r2);
    assert_eq!(r1.unwrap(), r2.unwrap());
}
