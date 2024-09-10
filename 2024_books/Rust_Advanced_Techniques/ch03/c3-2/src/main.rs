#![allow(dead_code)]
#![allow(unused_imports)]

use std::{cell::RefCell, fmt, rc::Rc};

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
	numbers.head.iter().for_each(|v| println!("--> v: {:?}", v.borrow()));

	numbers.reset_cursor();
	println!("--> numbers last: {:?}", numbers.last());

	let mut persons = LinkedList::new(Person::new("d2jvkpn".into()));

	while let Some(person) = persons.next() {
		person.borrow().hello();
		// let p = person.into_inner(); // Clone is required for T
		// p.hello();
	}
}

// #[derive(Debug)]
struct Node<T> {
	item: Rc<RefCell<T>>,
	next: NextNode<T>,
}

type NextNode<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> Node<T> {
	pub fn new(item: T) -> Self {
		Self { item: Rc::new(RefCell::new(item)), next: None }
	}

	pub fn push(&mut self, item: T) {
		match &self.next {
			None => self.next = Node::new(item).into(),
			Some(v) => v.borrow_mut().push(item),
		}
	}
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		let mut vec = Vec::new();
		vec.push(self.item.clone());

		let mut next = self.next.clone();
		while let Some(node) = next {
			vec.push(node.borrow().item.clone());
			next = node.borrow().next.clone();
		}

		write!(f, "{:?}", vec)
	}
}

impl<T> From<Node<T>> for NextNode<T> {
	fn from(node: Node<T>) -> Self {
		Some(Rc::new(RefCell::new(node)))
	}
}

#[derive(Debug)]
struct LinkedList<T> {
	head: NextNode<T>,
	size: usize,
	cursor: NextNode<T>,
}

impl<T> LinkedList<T> {
	fn new(t: T) -> Self {
		let head: NextNode<T> = Node::new(t).into();
		Self { head: head.clone(), size: 1, cursor: head.clone() }
	}

	pub fn push(&mut self, item: T) -> &mut Self {
		match &self.head {
			None => self.head = Node::new(item).into(),
			Some(v) => v.borrow_mut().push(item),
		};

		self.size += 1;
		self
	}

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn reset_cursor(&mut self) {
		self.cursor = self.head.clone();
	}
}

impl<T> Iterator for LinkedList<T> {
	type Item = Rc<RefCell<T>>;

	fn next(&mut self) -> Option<Self::Item> {
		let cursor = self.cursor.clone()?;
		let ans = cursor.borrow();
		self.cursor = ans.next.clone();
		Some(ans.item.clone())
	}
}

struct Person {
	name: String,
}

impl Person {
	pub fn new(name: String) -> Self {
		Self { name }
	}

	pub fn hello(&self) {
		println!("Hello, I'm {}.", self.name);
	}
}
