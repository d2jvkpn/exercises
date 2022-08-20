// cell.rs

use std::cell::Cell;

#[derive(Debug, Copy, Clone)]
struct Bag {
    item: u32,
}

fn main() {
    let bag = Cell::new(Bag { item: 1 });
    let hand1 = &bag;
    let hand2 = &bag;

    hand1.set(Bag { item: 2 });
    println!("{:?}, {}", bag, bag.get().item);

    hand2.set(Bag { item: 3 });
    println!("{:?}, {}", bag, bag.get().item);
}
