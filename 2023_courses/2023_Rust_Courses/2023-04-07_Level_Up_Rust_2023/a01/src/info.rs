use std::{ffi::CString, fmt::Display};

fn info<T: Display>(val: &T) {
    println!("THE TEXT: {}", val);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    // cargo test --lib -- t_info --show-output
    #[test]
    fn t_info() {
        let s1 = "abc";
        let s2 = "ABC".to_string();

        info(&s1);
        info(&s2);

        let s3 = CString::new("?").unwrap();
        info(&s3.to_string_lossy());

        let d = Path::new("/path/to/target");
        info(&d.display());
    }
}
