use x02::puzzle::Puzzle;

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
}
