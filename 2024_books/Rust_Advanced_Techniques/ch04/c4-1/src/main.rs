use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
	// println!("Hello, world!");

	let outer = Arc::new((Mutex::new(0), Condvar::new()));

	let inner = outer.clone();
	thread::spawn(move || {
		let (mutex, cond_var) = &*inner;
		let mut guard = mutex.lock().unwrap();
		*guard += 1;
		println!("==> inner guard={guard}");
		cond_var.notify_one();
	});

	let (mutex, cond_var) = &*outer;
	let mut guard = mutex.lock().unwrap();
	println!("==> outer before wait guard={guard}");

	if *guard == 0 {
		// println!("~~~");
		guard = cond_var.wait(guard).unwrap();
	}

	println!("==> outer after wait guard={guard}");
}
