#![allow(dead_code)]

use ch05::{basic, binary_search, gen, parallel};

fn main() {
    let data = gen::generate_random_data(16);
    // dbg!(&data);
    println!("==> data: {data:?}");

    let mut d1 = data.clone();
    basic::insertion_sort(&mut d1);
    assert!(gen::is_sorted(&d1));
    // println!("==> insertion_sort: {d1:?}");

    let mut d1 = data.clone();
    basic::selection_sort(&mut d1);
    assert!(gen::is_sorted(&d1));
    // println!("==> selection_sort: {d1:?}");

    let mut d1 = data.clone();
    basic::bubble_sort(&mut d1);
    assert!(gen::is_sorted(&d1));
    // println!("==> bubble_sort: {d1:?}");

    // println!("Hello, wrold!");
    let mut ans = vec![127, 9, 99, 10, 42, 12];

    parallel::merge_sort_v2(&mut ans);
    assert!(gen::is_sorted(&ans));

    assert_eq!(binary_search(&ans, 9), Some(0));
}
