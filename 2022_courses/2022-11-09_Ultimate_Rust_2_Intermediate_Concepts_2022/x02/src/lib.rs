use thiserror::Error;

pub mod cake;
pub mod puzzles;

use puzzles::Puzzle;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PuzzleError {
    #[error("Piece {0} doesn't fit!")]
    WontFit(u16),
    #[error("Missing a piece")]
    MissingPieces,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PuzzleError2 {
    #[error("Piece {index} doesn't fit!")]
    WontFit { index: u16 },
    #[error("Missing a piece")]
    MissingPieces,
}

pub fn fill(puzzle: &Puzzle) -> Result<(), PuzzleError> {
    if puzzle.num_pieces == 0 {
        return Err(PuzzleError::MissingPieces);
    }

    Ok(())
}

pub fn fill2(puzzle: &Puzzle) -> Result<Puzzle, PuzzleError> {
    fill(puzzle)?;

    let mut ans = puzzle.clone();
    ans.name = "Hello".into();
    Ok(ans)
}
