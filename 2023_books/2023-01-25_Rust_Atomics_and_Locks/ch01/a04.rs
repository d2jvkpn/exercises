use std::thread;

fn main() {
    //
    println!(">>>");
    static X: [i32; 3] = [1, 2, 3];

    let t1 = thread::spawn(|| dbg!(&X));
    let t2 = thread::spawn(|| dbg!(&X));

    let results = vec![t1.join(), t2.join()];
    results.into_iter().for_each(|v| {
        v.unwrap();
    });

    //
    println!(">>>");
    // This can be fine if it happens only a limited number of times
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    let t1 = thread::spawn(move || dbg!(x));
    let t2 = thread::spawn(move || dbg!(x));

    let results = vec![t1.join(), t2.join()];
    results.into_iter().for_each(|v| {
        v.unwrap();
    });
}
