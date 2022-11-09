/*!
Hi! I'm your friendly Rust Puzzle Library documenttion. Please come in, sit down, and have a cup of
hot chocolate!
*/

//! ![puzzle image](https://bkimg.cdn.bcebos.com/pic/6609c93d70cf3bc7ff07a991de00baa1cd112a43)

//! END

// comments that not show in doc

/**
Number of  pieces in the puzzle

# History

This is  separate paragraph.
- Clickable link: [`PUZZLE_PIECES`]
- [Clickable link](PUZZLE_PIECES)
- [Spawn a thread](std::thread::spawn)
- We tried `7`, but this is better
**/
pub const PUZZLE_PIECES: u32 = 42;

/// This is a Puzzle!
#[derive(Debug)]
pub struct Puzzle {
    /// Number of pieces
    pub num_pieces: u32,
    /// Descriptive name
    pub name: String,
}

impl Puzzle {
    /// Make a new Puzzle!
    pub fn new() -> Self {
        Self { num_pieces: PUZZLE_PIECES, name: "Forest Lake".into() }
    }
}
