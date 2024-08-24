#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;

fn main() {
	println!("Hello, world!");

	let my_poodle: Dog<Poodle> = Dog { name: "Jeffrey".into(), breed: PhantomData };
	// println!("{}", my_poodle.breed_name()); // Note: compile error

	let my_labrador: Dog<Labrador> = Dog { name: "Jeffrey".into(), breed: PhantomData };

	println!("==> My dog is a {}, named {}.", my_labrador.breed_name(), my_labrador.name,);
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
	fn breed_name(&self) -> &str {
		"labrador"
	}
}
impl Dog<Retriever> {
	fn breed_name(&self) -> &str {
		"retriever"
	}
}
