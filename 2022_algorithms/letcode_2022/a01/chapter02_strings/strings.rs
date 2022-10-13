fn first_uniq_char(s: String) -> i32 {
    use std::collections::HashMap;

    let mut count: HashMap<&char, usize> = HashMap::new();
    let chars: Vec<char> = s.chars().collect();

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

fn main() {
    //    println!(
    //        ">>> first_uniq_char: {}",
    //        first_uniq_char("loveleetcode".to_string())
    //    );

    assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
    println!(">>> first_uniq_char: ok")
}
