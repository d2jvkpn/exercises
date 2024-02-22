use std::thread;

fn main() {
    let count = thread::available_parallelism().unwrap().get();
    assert!(count >= 1_usize);
    println!("--> available_parallelism: {}", count);

    println!("--> num_cpus: {}", num_cpus::get());
}
