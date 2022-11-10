use x02::puzzles::Puzzle;

use x02::PuzzleError as PE;

fn main() {
    let puzzle = Puzzle { num_pieces: 42, name: "Forest Lake".into() };

    println!(
        "~~~ {}\n~~~ {}\n~~~ {}",
        puzzle,
        Puzzle::default(),
        Puzzle { num_pieces: 24, ..Default::default() },
    );

    let pieces: u16 = (&puzzle).into();
    assert_eq!(pieces, u16::from(&puzzle));

    println!("{:?}", call()); // Err(WontFit(8))

    if let Err(e) = call() {
        println!("{}", e); // Piece 8 doesn't fit!

        match e {
            PE::WontFit(_) => println!("......"),
            _ => {}
        }
    }
}

fn call() -> Result<(), PE> {
    Err(PE::WontFit(8))
}
