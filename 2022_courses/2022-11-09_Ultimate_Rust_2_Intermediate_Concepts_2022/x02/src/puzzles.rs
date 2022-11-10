#![allow(unused_imports)]

use std::cmp::{Eq, PartialEq};
use std::convert::{From, Into};
use std::default::Default;
use std::fmt;
use std::marker::{Copy, Send, Sized, Sync, Unpin};

#[derive(Clone, Debug)]
pub struct Puzzle {
    pub num_pieces: u16,
    pub name: String,
}

#[derive(Clone, Copy)]
pub enum PuzzleType {
    Jigsaw,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum PuzzleError {
    WontFit(u16),
    MissingPieces,
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

impl From<&Puzzle> for u16 {
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

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WontFit(v) => write!(f, "Piece {} doesn't fit!", v),
            Self::MissingPieces => write!(f, "Missing a piece"),
        }
    }
}
