fn first_uniq_char(s: String) -> i32 {
    use std::collections::HashMap;

    let chars: Vec<char> = s.chars().collect();
    let mut count: HashMap<&char, usize> = HashMap::with_capacity(chars.len());

    //    chars.iter().for_each(|c| {
    //        let val = count.entry(c).or_insert(0);
    //        *val += 1;
    //    });

    chars.iter().for_each(|c| *count.entry(c).or_insert(0) += 1);

    match chars.iter().position(|c| count[c] == 1) {
        Some(v) => v as i32,
        None => -1,
    }
}

fn reverse_int(mut x: i32) -> i32 {
    let mut res = 0;
    while x != 0 {
        if x > 0 && res > (i32::MAX - x % 10) / 10 {
            return 0;
        } else if x < 0 && res < (i32::MIN - x % 10) / 10 {
            return 0;
        }
        res = res * 10 + x % 10;
        x /= 10;
    }
    res
}

fn main() {
    //    println!(
    //        ">>> first_uniq_char: {}",
    //        first_uniq_char("loveleetcode".to_string())
    //    );

    assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
    println!(">>> first_uniq_char: ok");

    assert_eq!(reverse_int(123), 321);
    assert_eq!(reverse_int(-123), -321);
    println!("{}, {}", i32::MAX, -12 % 10);
    assert_eq!(reverse_int(1534236469), 0);
    assert_eq!(reverse_int(-1534236469), 0);
    println!(">>> reverse_int: ok");
}
