use anyhow::{Context, Result};
use thiserror::Error;

use std::cmp::PartialEq;
use std::convert::{From, Into};
use std::default::Default;
use std::io::Read;
use std::{fmt, fs};

#[derive(Clone, Debug)]
pub struct Puzzle {
    pub num_pieces: u16,
    pub name: String,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PuzzleError {
    #[error("Piece {0} doesn't fit!")]
    WontFit(u16),
    #[error("Missing a piece")]
    MissingPieces,
}

impl Puzzle {
    /// Make a new puzzle!
    pub fn new() -> Self {
        let puzzle = Default::default();
        puzzle
    }

    /// Load a puzzle from a file
    pub fn from_file(mut fh: fs::File, num_pieces: u16) -> Result<Self> {
        let mut name = String::new();
        fh.read_to_string(&mut name)?;
        name = String::from(name.trim());
        Ok(Self { num_pieces, name })
    }
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

// PuzzleError can be std::error::Error as impl Debug and Display
//impl fmt::Display for PuzzleError {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match self {
//            Self::WontFit(v) => write!(f, "Piece {} doesn't fit!", v),
//            Self::MissingPieces => write!(f, "Missing a piece"),
//        }
//    }
//}

pub fn get_puzzle(fp: &str, num_pieces: u16) -> Result<Puzzle> {
    // let fh = file::File::open(fp).context("couldn't open the puzzle file")?;
    let fh =
        fs::File::open(fp).with_context(|| format!("couldn't open the puzzle file: {}", fp))?;

    // Puzzle{name: fs::read_to_string(fp)?, num_pieces: 42};

    let puzzle =
        Puzzle::from_file(fh, num_pieces).context("couldn't convert data into a puzzle")?;
    Ok(puzzle)
}
