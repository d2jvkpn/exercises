#![allow(dead_code)]
#![allow(unused_imports)]

mod linked_list;

use linked_list::LinkedList;
use std::{
	cell::{Ref, RefCell, RefMut},
	fmt,
	rc::Rc,
};

fn main() {
	// println!("Hello, world!");

	// 1.
	let bark = || println!("Bark!");
	bark();

	let increment = |value| {
		println!("{value} will be incremented and returned");
		value + 1
	};

	println!("{}", increment(5));

	// 2. FnOnce
	let consumable = String::from("cookie");
	let consumer = move || {
		println!("{}", consumable);
	};
	consumer();
	consumer();

	// println!("{}", consumable); // ERROR: value borrowed here after move

	// 3. Fn
	let x = 10;
	let add_fn = |y| x + y; // `x` is captured by reference

	println!("add_fn={}, x={}", add_fn(5), x); // Prints 15, 10
	println!("add_fn={}, x={}", add_fn(5), x); // Prints 15, 10

	let x = vec![1, 2, 3]; // `x` is captured by value
	let consume_x = move || {
		println!("{:?}", x); // Consumes `x`
	};

	consume_x(); // Prints [1, 2, 3]
	consume_x(); // Prints [1, 2, 3]

	// 4. FnMut
	let mut x = 10;
	let mut add_fn_mut = |y| {
		x += y; // `x` is captured by mutable reference
		x
	};

	println!("add_fn_mut={}", add_fn_mut(5)); // Prints 15
	println!("add_fn_mut={}", add_fn_mut(10)); // Prints 25

	// 5. FnOnce
	fn take_closure<F>(clo: F)
	where
		F: FnOnce(u32) -> Vec<u32>,
	{
		let vec = clo(12);
		println!("modified vector is {:?}", vec);
		// let value2 = clo(32); // second call is not allowed throws an error
	}

	let mut v: Vec<u32> = vec![1];
	take_closure(|x: u32| {
		v.push(x);
		v
	});

	// take_closure(...); // value used here after move

	// 6. LinkedList
	let mut numbers = LinkedList::new(1);
	numbers.push(2).push(3).push(4).push(5);
	assert_eq!(numbers.size(), 5);

	println!("==> numbers: {numbers:?}");

	while let Some(item) = numbers.next() {
		println!("--> 1. item: {}", item.borrow());
	}

	numbers.reset_cursor();
	while let Some(item) = numbers.next() {
		println!("--> 2. item: {}", item.borrow());
	}

	numbers.reset_cursor();
	numbers.get_head().iter().for_each(|v| println!("--> iter v: {:?}", v.borrow()));

	numbers.reset_cursor();
	println!("--> numbers last: {:?}", numbers.last());

	//
	let mut persons = LinkedList::new(Person::new("d2jvkpn".into()));
	persons.push("alice".into());

	while let Some(person) = persons.next() {
		person.borrow().hello();
		// let p = person.into_inner(); // Clone is required for T
		// p.hello();
	}

	// 7. smart pointers
	let rc_name: Rc<RefCell<String>> = Rc::new(RefCell::new("d2jvkpn".to_string()));
	// dbg!(&rc_name);

	{
		let name: Ref<String> = rc_name.borrow();
		println!("==> 1. name: {name}"); // 1. name: d2jvkpn
	}

	{
		let mut name: RefMut<String> = rc_name.borrow_mut();
		*name = "D2JVKPN".to_string();
	}
	// dbg!(&rc_name);

	let name: String = <RefCell<String> as Clone>::clone(&rc_name).into_inner();
	println!("==> 2. name: {name}"); // 2. name: D2JVKPN

	//
	let rc_name: Rc<RefCell<String>> = Rc::new(RefCell::new("d2jvkpn".to_string()));

	let name: Result<String, Rc<RefCell<String>>> = Rc::try_unwrap(rc_name).map(|v| v.into_inner());

	println!("==> 3. name: {name:?}"); // 3. name: Ok("d2jvkpn")

	// 8. iter
	let arr = ["duck", "1", "2", "goose", "3", "4"];

	let ans: Vec<i32> = arr.iter().flat_map(|v| v.parse::<i32>()).collect();
	println!("{:?}", ans); // [1, 2, 3, 4]

	let (successes, failures): (Vec<_>, Vec<_>) =
		arr.iter().map(|v| v.parse::<i32>()).partition(Result::is_ok);

	println!("successses={:?}", successes); // successses=[Ok(1), Ok(2), Ok(3), Ok(4)]
	println!("failures={:?}", failures);
	/*
	failures=[Err(ParseIntError { kind: InvalidDigit }), Err(ParseIntError { kind: InvalidDigit})]
	*/

	let successes: Vec<_> = successes.into_iter().map(Result::unwrap).collect();
	let failures: Vec<_> = failures.into_iter().map(Result::unwrap_err).collect();

	println!("successses={:?}", successes); // successses=[1, 2, 3, 4]
	println!("failures={:?}", failures);
	// failures=[ParseIntError { kind: InvalidDigit }, ParseIntError { kind: InvalidDigit }]

	let popular_dog_breeds = vec![
		"Labrador",
		"French Bulldog",
		"Golden Retriever",
		"German Shepherd",
		"Poodle",
		"Bulldog",
		"Beagle",
		"Rottweiler",
		"Pointer",
		"Dachshund",
	];
	let ranked_breeds: Vec<_> = popular_dog_breeds.clone().into_iter().enumerate().collect();
	println!("{:?}", ranked_breeds); // [(0, "Labrador"), (1, "French Bulldog")...]

	let ranked_breeds: Vec<_> = popular_dog_breeds.clone()
		.into_iter()
		.enumerate()
		.map(|(idx, breed)| (idx + 1, breed))
		.collect();

	println!("{:?}", ranked_breeds);
}

struct Person {
	name: String,
}

impl From<&str> for Person {
	fn from(name: &str) -> Self {
		Self { name: name.to_string() }
	}
}

impl Person {
	pub fn new(name: String) -> Self {
		Self { name }
	}

	pub fn hello(&self) {
		println!("Hello, I'm {}.", self.name);
	}
}
