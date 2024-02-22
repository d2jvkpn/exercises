use std::thread;

fn main() {
    start_scoped_thread();
}

fn start_scoped_thread() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread: {:?}", thread::current().id());
            dbg!(&a);
        });

        s.spawn(|| {
            println!("hello from the second scoped thread: {:?}", thread::current().id());
            x = a[0] + a[2];
        });

        println!("hello from the main thread: {:?}", thread::current().id());
    });

    a.push(4);

    assert_eq!(x, a.len());
}
