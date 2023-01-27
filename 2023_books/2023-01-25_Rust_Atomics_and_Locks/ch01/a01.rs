use std::thread;

fn main() {
    let t1 = thread::spawn(hello);
    let t2 = thread::spawn(hello);
    println!("Hello from the main thread.");

    let results: Vec<thread::Result<_>> = vec![t1.join(), t2.join()];
    results.into_iter().for_each(|v| v.unwrap())
}

fn hello() {
	// println macro uses std::io::Stdout::lock()
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
