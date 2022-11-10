#![allow(unused_imports)]

pub mod puzzles;

use puzzles::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_get_puzzle() {
        let puzzle = get_puzzle("hello.txt", 42).unwrap();
        assert_eq!(puzzle, Puzzle { name: "Hello, world!".into(), num_pieces: 42 });
    }
}
