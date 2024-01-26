use std::{sync::mpsc, thread};

fn main() {
    let (sender, receiver) = mpsc::channel();

    for i in 0..5 {
        let sender = sender.clone();

        thread::spawn(move || {
            sender.send(i).unwrap();
        });
    }

    for _ in 0..5 {
        let ans = receiver.recv().unwrap();
        println!("==> Received: {}", ans);
    }
}
