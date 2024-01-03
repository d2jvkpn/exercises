#![allow(unused_variables)]

fn main() {
    let a: i8 = -2;
    let b: f32 = 3.14;
    let c: bool = false;
    let d: char = 'a';

    let x: (i32, f64, u8) = (42, 5.32, 1);
    let x_i32 = x.0;
    let x_f64 = x.1;
    let x_u8 = x.2;

    let val = 72_u64;

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "Auguest",
        "September",
        "October",
        "November",
        "December",
    ];

    let first_month = months[0];
    let half_year = &months[..6];

    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    println!("Months: {:?}\n, Days: {:?}", months, days);

    // Linear collection types: String, Vec, VecDeque, LinkedList
    // Nonlinear collection types: HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap
    // Big O: get(), insert(i), remove(i), append(), split_off(i)
}
