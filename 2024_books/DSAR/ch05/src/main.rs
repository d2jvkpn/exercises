#![allow(dead_code)]

use ch05::{basic, binary_search, gen, heap_sort, parallel};

fn main() {
    let input = gen::generate_random_data(16);
    // dbg!(&input);
    println!("==> input: {input:?}");

    let mut d1 = input.clone();
    basic::insertion_sort(&mut d1);
    assert!(gen::is_sorted(&d1));
    // println!("==> insertion_sort: {d1:?}");

    let mut d1 = input.clone();
    basic::selection_sort(&mut d1);
    assert!(gen::is_sorted(&d1));
    // println!("==> selection_sort: {d1:?}");

    let mut d1 = input.clone();
    basic::bubble_sort(&mut d1);
    assert!(gen::is_sorted(&d1));
    // println!("==> bubble_sort: {d1:?}");

    // println!("Hello, wrold!");
    let ans = vec![127, 9, 99, 10, 42, 12];

    let mut d1 = ans.clone();
    parallel::merge_sort_v2(&mut d1);
    assert!(gen::is_sorted(&d1));

    assert_eq!(binary_search(&d1, 9), Some(0));

    let mut d1 = input.clone();
    heap_sort(&mut d1);
    assert!(gen::is_sorted(&d1));
}
