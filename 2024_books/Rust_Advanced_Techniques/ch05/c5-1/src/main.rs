#![feature(trace_macros)]
#![allow(dead_code)]

macro_rules! print_what_it_is {
	() => {
		println!("A macro with no arguments")
	};

	($e:expr) => {
		println!("A macro with an expression")
	};

	($s:stmt) => {
		println!("A macro with a statement")
	};

	($e:expr, $s:stmt) => {
		println!("An expression followed by a statement")
	};
}

macro_rules! special_println {
	($($arg:tt)*) => { // tt: token trees
		// println!($($arg)*)
		// println!("==> Printed specially: {}, {}.", $($arg)*)
		println!("==> Printed specially: {}", format!($($arg)*))
	};
}

// separated list of identifiers
macro_rules! var_print {
	($($v:ident),*) => {
		println!(concat!("~~~ ", $(stringify!($v),"={:?}, "),*), $($v),*)
	};
}

macro_rules! dog_struct {
	($breed:ident) => {
		struct $breed {
			name: String,
			age: i32,
			breed: String,
		}

		impl $breed {
			fn new(name: &str, age: i32) -> Self {
				Self { name: name.into(), age, breed: stringify!($breed).into() }
			}
		}

		impl Dog for $breed {
			fn name(&self) -> &String {
				&self.name
			}

			fn age(&self) -> i32 {
				self.age
			}

			fn breed(&self) -> &String {
				&self.breed
			}
		}
	};
}

fn main() {
	print_what_it_is!();
	print_what_it_is!({});
	print_what_it_is!(;);
	print_what_it_is!({}, ;);

	trace_macros!(true);
	// special_println!("hello: {:?}", "world");
	// special_println!("hello", "world");
	special_println!("hello: {}", "world");
	trace_macros!(false);

	let counter = 7;
	let gauge = std::f64::consts::PI;
	let name = "Peter";
	var_print!(counter, gauge, name);

	dbg!(counter, gauge, &name);

	dog_struct!(Labrador);
	dog_struct!(Golden);
	dog_struct!(Poodle);

	let peter = Poodle::new("Peter", 7);
	println!("{} is a {} of age {}.", peter.name(), peter.breed(), peter.age());
}

trait Dog {
	fn name(&self) -> &String;
	fn age(&self) -> i32;
	fn breed(&self) -> &String;
}
