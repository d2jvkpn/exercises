// const PI: f32 = 3.141_592_7;

/// the main function
fn main() {
    println!("Hello, world!");
    println!(">>> count_to_5: {}", count_to_5());

    let aa = 1..100;
    println!(">>> count: {}", aa.count());

    println!(">>> {0:?}, {0:#?}", x01::puzzle::Puzzle::new());

    // std::ops::{Fn, FnMut, FnOnce}
    let s = "hello";
    let f = || {
        println!("s={:?}", s);
    };

    let s2 = s.clone();
    let f2 = move || {
        println!("s2={:?}", s2);
    };

    f(); // s is available as it's reference
    f2(); // s2 isn't available
    println!("s={:?}", s);

    let nums = vec![6, 7, 8, 9];

    for num in &nums {
        println!("~~~ {}", num);
    }
    println!("{:?}", nums);

    // iterator adaptors: for_each, map, filter;
    // iterator consumer: sum, collect;
    nums.iter().for_each(|v| println!("num={}", v));

    let total: i32 = nums.into_iter().map(|v| v * 3).filter(|v| v % 2 == 0).sum();
    dbg!(&total);
    // dbg!(&nums); // nums.into_iter() consumed nums

    let mut nums = vec![6, 7, 8];
    let nums2: Vec<i32> = nums.iter().map(|v| v * 3).filter(|x| x % 2 == 0).collect();
    dbg!(&nums2);

    nums.iter_mut().for_each(|v| *v *= 3);
    dbg!(&nums);

    let mut chars = vec!['a', 'b', 'c'];
    let mut drain = chars.drain(1..);

    assert_eq!(drain.next().unwrap(), 'b');
    dbg!(&drain);
    drop(drain); // compile will failed without this line
    dbg!(&chars);

    let mut nums = vec![1, 2, 3, 4];
    nums.iter_mut().for_each(|v| *v *= 3);
    dbg!(&nums);

    let words = vec!["autobot", "beach", "car", "decepticon", "energon", "fronthy"];

    let transformed: Vec<String> =
        words.into_iter().filter(|v| !v.contains("h")).map(|v| v.to_uppercase()).collect();

    dbg!(&transformed);
}

/// count to 5 and return
fn count_to_5() -> i32 {
    let mut num = 0;

    loop {
        // if num > PI as i32 {
        if num > std::f32::consts::PI as i32 {
            break;
        }
        num += 1;
    }

    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_count_to_5() {
        assert_eq!(count_to_5(), 5);
    }
}
