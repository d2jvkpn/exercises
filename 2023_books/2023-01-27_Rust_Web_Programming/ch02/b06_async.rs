use std::{thread, time};

fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);

    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

fn main() {
    let now = time::Instant::now();

    let thread_one: thread::JoinHandle<i8> = thread::spawn(|| do_something(1));
    let thread_two: thread::JoinHandle<i8> = thread::spawn(|| do_something(2));
    let thread_three: thread::JoinHandle<i8> = thread::spawn(|| do_something(3));

    let result_one = thread_one.join();
    let result_two = thread_two.join();
    let result_three = thread_three.join();

    println!(
        "time elapsed {:?}, result {}",
        now.elapsed(),
        result_one.unwrap() + result_two.unwrap() + result_three.unwrap()
    );
}


/*
match thread_result {
	Ok(v) => {
		println!("the result for {} is {}", v, name);
	}
	Err(e) => {
		if let Some(v) = e.downcast_ref::<String>() {
			println!("the error for {} is: {}", name, v);
		} else {
			println!("there error for {} does not have a message", name);
		}
	}
}
/*
