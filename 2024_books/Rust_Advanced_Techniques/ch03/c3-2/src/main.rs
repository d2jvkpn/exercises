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
	fn take_closure<F> (mut clo: F) where F: FnOnce(u32) -> Vec<u32> {
		let vec = clo(12);
		println!("modified vector is {:?}", vec);
		// let value2 = clo(32); // second call is not allowed throws an error
	}

	let mut v: Vec<u32> = vec![1];
	take_closure(|x: u32| {
		v.push(x);
		v
	});
}
