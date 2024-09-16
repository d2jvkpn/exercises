use lazy_static::lazy_static;
use once_cell::sync::{Lazy, OnceCell};

use std::{
	sync::{Arc, Mutex},
	thread,
};

lazy_static! {
	static ref POPULAR_BABY_NAMES_2020: Vec<String> = {
		vec![
			String::from("Olivia"),
			String::from("Liam"),
			String::from("Emma"),
			String::from("Noah"),
		]
	};
}

static POPULAR_BABY_NAMES_2019: Lazy<Vec<String>> = Lazy::new(|| {
	vec![String::from("Olivia"), String::from("Liam"), String::from("Emma"), String::from("Noah")]
});

static SETTINGS: OnceCell<String> = OnceCell::new();

fn main() {
	// println!("Hello, world!");

	// 1.
	let cell = std::cell::OnceCell::new(); // Cannot be shared between threads safely
	assert!(cell.get().is_none());

	let value: &String = cell.get_or_init(|| "Hello, World!".to_string());
	assert_eq!(value, "Hello, World!");
	assert!(cell.get().is_some());

	let value: &String = cell.get_or_init(|| "42".to_string());
	assert_eq!(value, "Hello, World!");
	assert!(cell.get().is_some());

	// 2.
	thread_local! {
		static POPULAR_BABY_NAMES_2021: Arc<Mutex<Option<Vec<String>>>> =
			Arc::new(Mutex::new(None));
	};

	let arc = POPULAR_BABY_NAMES_2021.with(|arc| arc.clone());
	let mut inner = arc.lock().expect("unable to lock mutex");

	*inner = Some(vec![
		String::from("Olivia"),
		String::from("Liam"),
		String::from("Emma"),
		String::from("Noah"),
	]);

	// 3.
	println!("==> 3.1. popular baby names of 2020: {:?}", *POPULAR_BABY_NAMES_2020);

	let thread_1 = thread::spawn(move || {
		println!("==> 3.2. popular baby names of 2019: {:?}", *POPULAR_BABY_NAMES_2019);
	});

	let thread_2 = thread::spawn(move || {
		println!("==> 3.3. popular baby names of 2019: {:?}", *POPULAR_BABY_NAMES_2019);
	});

	thread_1.join().unwrap();
	thread_2.join().unwrap();

	// 4.
	SETTINGS.set("Hello".to_string()).unwrap();

	let thread_1 = thread::spawn(|| {
		let ans = SETTINGS.get_or_init(|| "42".to_string());
		println!("==> 4.1. SETTINGS: {:?}, ans={ans}", SETTINGS.get());
	});

	let thread_2 = thread::spawn(|| {
		println!("==> 4.2. SETTINGS: {:?}", SETTINGS.get());
	});

	thread_1.join().unwrap();
	thread_2.join().unwrap();
}
