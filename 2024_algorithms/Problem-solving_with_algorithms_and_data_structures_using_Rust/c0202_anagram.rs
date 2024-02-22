use std::collections::HashMap;

fn main() {
    // println!("Hello, wrold!");
    assert!(anagram1("rust", "trus"));
    assert!(anagram3("rust", "trus"));
    assert!(anagram4("rust", "trus"));
}

fn anagram1(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let v1: Vec<Option<char>> = s1.chars().map(|v| Some(v)).collect();
    let mut v2: Vec<Option<char>> = s2.chars().map(|v| Some(v)).collect();
    let mut ch;

    'LOOP: for i in 0..v1.len() {
        ch = v1[i];

        for j in 0..v2.len() {
            if v2[j] == ch {
                v2[j] = None;
                continue 'LOOP;
            }
        }

        return false;
    }

    true
}

/*
// anagram2:
v1.sort();
v2.sort();
v1 == v2
*/

// for a-z only
fn anagram3(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut c1 = [0; 26];
    let mut c2 = [0; 26];

    s1.chars().for_each(|v| c1[v as usize - 97] += 1);
    s2.chars().for_each(|v| c2[v as usize - 97] += 1);

    c1 == c2

    /*
    // anagram4:
    let mut hm1 = HashMap::with_capacity(64);
    let mut hm2 = HashMap::with_capacity(64);

    s1.chars().for_each(|v| hm1.entry(v).or_insert_with(0) += 1);
    s1.chars().for_each(|v| hm1.entry(v).or_insert_with(0) += 1);

    v1 == v2
    */
}

fn anagram4(s1: &str, s2: &str) -> bool {
    let cap = s1.chars().count();

    if s2.chars().count() != cap {
        return false;
    }

    let mut hm1 = HashMap::with_capacity(cap);
    let mut hm2 = HashMap::with_capacity(cap);

    s1.chars().for_each(|v| *hm1.entry(v).or_insert_with(|| 0) += 1);
    s1.chars().for_each(|v| *hm2.entry(v).or_insert_with(|| 0) += 1);

    hm1 == hm2
}
