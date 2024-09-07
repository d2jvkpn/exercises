#![allow(dead_code)]

use std::{
	fs::{create_dir_all, File},
	io::prelude::*,
};

fn main() {
	// println!("Hello, world!");

	match_with_guard(1, false);

	match_enum_types(&DistinctTypes::Name("d2jvkpn".to_string()));

	let black_cat = Cat { name: String::from("Henry"), color: CatColor::Black };
	let cheshire_cat = Cat { name: String::from("Penelope"), color: CatColor::Cheshire };
	match_on_black_cats(&black_cat);
	match_on_black_cats(&cheshire_cat);

	// write_to_file().unwrap();
	try_to_write_to_file();
}

// 1.
fn match_with_guard(value: i32, choose_first: bool) {
	match value {
		v if v == 1 && choose_first => {
			println!("First match: This value is equal to 1")
		},
		v if v == 1 && !choose_first => {
			println!("Second match: This value is equal to 1")
		},
		v if choose_first => {
			println!("First match: This value is equal to {v}")
		},
		v if !choose_first => {
			println!("Second match: This value is equal to {v}")
		},
		_ => println!("Fell through to the default case"),
	}
}

// 2.
enum DistinctTypes {
	Name(String),
	Count(i32),
}

fn match_enum_types(enum_types: &DistinctTypes) {
	match enum_types {
		DistinctTypes::Name(name) => println!("name={name}"),
		DistinctTypes::Count(count) => println!("count={count}"),
	}
}

// 3.
enum CatColor {
	Black,
	Red,
	Chocolate,
	Cinnamon,
	Blue,
	Cream,
	Cheshire,
}

struct Cat {
	name: String,
	color: CatColor,
}

fn match_on_black_cats(cat: &Cat) {
	match cat {
		Cat { name, color: CatColor::Black } => println!("This is a black cat named {name}"),
		Cat { name, color: _ } => println!("{name} is not a black cat"),
	}
}

// 4.
enum ErrorTypes {
	IoError(std::io::Error),
	FormatError(std::fmt::Error),
}

struct ErrorWrapper {
	source: ErrorTypes,
	message: String,
}

impl From<std::io::Error> for ErrorWrapper {
	fn from(source: std::io::Error) -> Self {
		Self { source: ErrorTypes::IoError(source), message: "there was an IO error!".into() }
	}
}

// fn write_to_file() -> std::io::Result<()> {
fn write_to_file() -> Result<(), ErrorWrapper> {
	create_dir_all("data")?;

	let mut file = File::create("data/test01.txt")?;
	file.write_all(b"File contents\n")?;
	Ok(())
}

fn try_to_write_to_file() {
	match write_to_file() {
		Ok(()) => println!("Write succeeded"),
		// Err(err) => println!("Write failed: {}", err.to_string()),
		Err(err) => println!("Write failed: {}", err.message),
	}
}
