#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;

fn main() {
	println!("Hello, world!");

	let my_poodle: Dog<Poodle> = Dog { name: "Jeffrey".into(), breed: PhantomData };
	// println!("{}", my_poodle.breed_name()); // Note: compile error

	// let my_labrador: Dog<Labrador> = Dog { name: "Jeffrey".into(), breed: PhantomData };
	let my_labrador: Dog<Labrador> = Dog::new("Jeffrey");
	println!("==> My dog is a {}, named {}.", my_labrador.breed_name(), my_labrador.name);

	let my_labrador: Dog<Labrador> = Dog::new("Jeffrey".to_string());
	println!("==> My dog is a {}, named {}.", my_labrador.breed_name(), my_labrador.name);
}

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
