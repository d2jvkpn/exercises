#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug, PartialEq)]
enum Card {
    King(Suit),
    Queen(Suit),
    Jack(Suit),
    Ace(Suit),
    Pip(Suit, usize), // 1..=10
}

impl Card {
    // self: &Self
    fn equals(&self, card: &Card) -> bool {
        self == card
    }
}

fn main() {
    let card1 = Card::Pip(Suit::Hearts, 10);
    let card2 = Card::King(Suit::Clubs);

    dbg!(&card1, &card2);

    println!("{}, {}", card1.equals(&card2), card1 == card2);

    println!("card1 = {:?}, card2 = {:?}", card1, card2);
}
