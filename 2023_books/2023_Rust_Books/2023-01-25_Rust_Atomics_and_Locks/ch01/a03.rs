use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        // imutable borrow across threads
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });

        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}
