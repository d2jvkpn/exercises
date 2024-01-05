mod c0307_deque;

use c0307_deque::*;

fn main() {
    assert!(is_palindrome("rustsur"));
    assert!(!is_palindrome("hello"));
}

pub fn is_palindrome(pal: &str) -> bool {
    let mut deque = Deque::new(pal.len());

    pal.chars().for_each(|v| {
        let _ = deque.push_back(v);
    });

    while let (Some(a), Some(b)) = (deque.pop_front(), deque.pop_back()) {
        if a != b {
            return false;
        }
    }

    true
}
