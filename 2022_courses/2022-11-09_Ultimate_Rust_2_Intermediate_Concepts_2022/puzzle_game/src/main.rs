use puzzle_game::puzzles;

use std::env;

fn main() {
    // println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("please provide a filepath!");
    }

    println!("{}", puzzles::get_puzzle(&args[1][..], 42).expect("!!!"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn t_42() {
        println!(
            "The Answer to the Ultimate Question of Life, the Universe, and Everything is 42."
        );
        assert_eq!(24 + 18, 42);
    }
}
