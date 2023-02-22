#![allow(dead_code)]

use chrono::{Local, SecondsFormat};

fn main() {
    println!(
        "Hello, world! {}",
        Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
    );

    // use a func in lib
    ch01::call_a::hello();
}

#[cfg(test)]
mod tests {
    use super::get_process_id;

    #[test]
    fn test_if_process_id_is_returned() {
        assert_ne!(get_process_id(), 0, "There is error in code");
    }
}
