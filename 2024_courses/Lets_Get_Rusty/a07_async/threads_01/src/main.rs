use std::{
    sync::{mpsc, Arc, Mutex, RwLock},
    thread,
    time::Duration,
};
// use std::rc::Rc;

fn main() {
    println!("Hello, world!");

    // Thread
    let handle = thread::spawn(|| {
        for i in 0..20 {
            println!("~~~ spawned thread: {i}");
        }
    });

    for i in 0..10 {
        println!("~~~ main thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // Thread
    let s: String = "Let's Get Rusty!".to_owned();

    let handle = thread::spawn(move || println!("~~~ {s}")); // moved into the thread

    handle.join().unwrap();

    // Passing messages through channel
    let (tx, rx) = mpsc::channel();

    let sentences = [
        "!dlroW olleH".to_owned(),
        ".tsurT eW tsuR nI".to_owned(),
        "!tsuR teG s'teL".to_owned(),
        "!tsuB ro tsuR".to_owned(),
    ];

    for s in sentences {
        let tx1 = tx.clone();
        thread::spawn(move || {
            let reversed: String = s.chars().rev().collect();
            tx1.send(reversed).unwrap();
        });
    }
    drop(tx);

    while let Ok(v) = rx.recv() {
        println!("==> Get {v}");
    }

    // Sharing status between threads by using Mutex
    let db = Arc::new(Mutex::new(Database::new()));
    let mut handles = Vec::new();

    for i in 0..16 {
        let db = db.clone();
        let handle = thread::spawn(move || {
            let mut db = db.lock().unwrap();
            db.connect(i);
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|v| {
        if let Err(e) = v.join() {
            eprintln!("!!! join thread error: {e:?}");
        }
    });

    println!("~~~ Mutex {:?}", db.lock().unwrap());

    // Sharing status between threads by using RwLock
    let db = Arc::new(RwLock::new(Database::new()));
    let mut handles = Vec::new();

    for i in 0..16 {
        let db = db.clone();
        let handle = thread::spawn(move || {
            let mut db = db.write().unwrap();
            db.connect(i);
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|v| {
        if let Err(e) = v.join() {
            eprintln!("!!! join thread error: {e:?}");
        }
    });

    println!("~~~ RwLock {:?}", db.read().unwrap());
}

#[derive(Debug)]
struct Database {
    connections: Vec<u32>,
}

impl Database {
    fn new() -> Self {
        Database { connections: Vec::with_capacity(42) }
    }

    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}
