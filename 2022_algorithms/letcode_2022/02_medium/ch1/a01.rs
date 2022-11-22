use std::collections::HashMap;

fn main() {
    let strs: Vec<String> =
        vec!["eat".into(), "tea".into(), "tan".into(), "ate".into(), "nat".into(), "bat".into()];

    let mut data: HashMap<Vec<char>, Vec<String>> = HashMap::new();

    strs.into_iter().for_each(|v| {
        let mut key: Vec<char> = v.chars().collect();
        key.sort();

        let d = data.entry(key).or_insert(Vec::new());
        d.push(v);
    });

    println!("{:?}", data.into_values());
}
