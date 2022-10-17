pub fn partition(s: String) -> Vec<Vec<String>> {
    let chars = s.chars().collect::<Vec<char>>();
    let len = chars.len();

    let mut result = Vec::new();

    for i in 0..len {
        let t = vec![chars[(i)..(i + 1)].iter().collect()];
        result.push(t);
    }

    result
}

/*
    cargo test -- partition --show-output
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn _partition() {
        let result = partition("hello".to_string());
        println!("{:?}", result);
        assert!(result.len() == 5);
    }
}
