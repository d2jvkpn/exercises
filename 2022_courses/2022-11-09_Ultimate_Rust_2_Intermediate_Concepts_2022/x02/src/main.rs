#![allow(unused_imports)]

use std::cmp::{Eq, PartialEq};
use std::convert::{From, Into};
use std::default::Default;
use std::fmt;
use std::marker::{Copy, Send, Sized, Sync, Unpin};

#[derive(Clone, Debug)]
pub struct Puzzle {
    pub num_pieces: u32,
    pub name: String,
}

#[derive(Clone, Copy)]
pub enum PuzzleType {
    Jigsaw,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "num_pieces={}, name={:?}", self.num_pieces, self.name)
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Puzzle { num_pieces: 2022, name: "Unknown".into() }
    }
}

impl From<&Puzzle> for u32 {
    fn from(item: &Puzzle) -> Self {
        item.num_pieces
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        // todo!()
        (self.num_pieces == other.num_pieces)
            && (self.name.to_lowercase() == other.name.to_lowercase())
    }
}

fn main() {
    let puzzle = Puzzle { num_pieces: 42, name: "Forest Lake".into() };

    println!(
        "~~~ {}\n~~~ {}\n~~~ {}",
        puzzle,
        Puzzle::default(),
        Puzzle { num_pieces: 24, ..Default::default() },
    );

    let pieces: u32 = (&puzzle).into();
    assert_eq!(pieces, u32::from(&puzzle));
}
