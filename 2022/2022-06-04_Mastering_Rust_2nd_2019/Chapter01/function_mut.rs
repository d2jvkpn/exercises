// function_mut.rs

fn increase_by(val: &mut u32, how_much: u32) {
    *val += how_much;
    println!("You made {} points", val);
}

fn main() {
    let mut score = 2048;
    increase_by(&mut score, 30);
    println!("score={}", score);
}
