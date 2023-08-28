mod m01;

use m01::m02;
use std::{net::Ipv4Addr, str::FromStr}; // Imported to use Ipv4Addr::from_str

fn main() {
    // println!("Hello, world!");
    m02::hello();

    //
    let ip_address = Ipv4Addr::from_str("127.0.0.1").unwrap();
    let string_proper = "String proper".to_owned();
    let string_slice = "string slice";
    needs_string(string_slice);
    needs_string("Literal string");
    needs_string(string_proper);

    needs_string(ip_address);
    needs_string_v2(ip_address);

    needs_string_v3("hello");
    needs_string_v3("hello".to_string());
}

fn needs_string<T: ToString>(almost_string: T) {
    let real_string = almost_string.to_string();
    println!("{}", real_string);
}

fn needs_string_v2(almost_string: impl ToString) {
    let real_string = almost_string.to_string();
    println!("{}", real_string);
}

fn needs_string_v3<T: AsRef<str>>(almost_string: T) {
    let real_string = almost_string.as_ref().to_owned();
    println!("{}", real_string);
}
