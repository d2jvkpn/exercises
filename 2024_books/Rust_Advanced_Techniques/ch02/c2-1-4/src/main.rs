#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;

fn main() {
	println!("Hello, world!");

	// 1.
	let my_poodle: Dog<Poodle> = Dog { name: "Jeffrey".into(), breed: PhantomData };
	// println!("{}", my_poodle.breed_name()); // Note: compile error

	// let my_labrador: Dog<Labrador> = Dog { name: "Jeffrey".into(), breed: PhantomData };
	let my_labrador: Dog<Labrador> = Dog::new("Jeffrey");
	println!("==> My dog is a {}, named {}.", my_labrador.breed_name(), my_labrador.name);

	let my_labrador: Dog<Labrador> = Dog::new("Jeffrey".to_string());
	println!("==> My dog is a {}, named {}.", my_labrador.breed_name(), my_labrador.name);

	// 2.
	let cat = Cat;
	println!("==> I am a {}.", describe_type(&cat));

	// 3.
	println!("==> {}.", hello_type::<Human>());
}

// 1. PhantomData
struct Labrador {}
struct Retriever {}
struct Poodle {}
struct Dachshund {}

struct Dog<Breed> {
	name: String,
	breed: PhantomData<Breed>,
}

impl Dog<Labrador> {
	pub fn new(name: impl AsRef<str>) -> Self {
		Self { name: name.as_ref().into(), breed: PhantomData }
	}

	fn breed_name(&self) -> &str {
		"labrador"
	}
}
impl Dog<Retriever> {
	fn breed_name(&self) -> &str {
		"retriever"
	}
}

// 2. generics and traits
pub trait SelfDescribing {
	fn describe(&self) -> String;
}

fn describe_type<T: SelfDescribing>(t: &T) -> String {
	t.describe()
}
// fn describe_type<T>(t: &T) -> String where T: SelfDescribing { t.describe() }

struct Cat;

impl SelfDescribing for Cat {
	fn describe(&self) -> String {
		"curious cat".into() // "happy little dog".into()
	}
}

// 3.
pub trait Hello {
	fn hello() -> String;
}

fn hello_type<T: Hello>() -> String {
	T::hello()
}

struct Human;

impl Hello for Human {
	fn hello() -> String {
		"Hello".into()
	}
}
