use std::{thread, time::Duration};

fn main() {
    println!("So, we start the program here!");

    let t1 = thread::spawn(move || {
        // Sleeping is essentially the same as yielding to the OS scheduler with a request to be
        // re-scheduled to run after a certain time has passed
        thread::sleep(Duration::from_millis(200));
        println!("The long running tasks finish last!");
    });

    let t2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));

        println!("We can chain callbacks...");
        let t3 = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            println!("...like this!");
        });
        t3.join().unwrap();
    });

    println!("The tasks run concurrently!");

    t1.join().unwrap();
    t2.join().unwrap();
}

/*
M:N model
M -> tasks created by thread::spawn
N -> OS shreads
*/
