fn slice_to_max_num(slice: &[usize]) -> String {
    let mut vec: Vec<String> = slice.iter().map(|&v| v.to_string()).collect();

    // "301" > "30", while "30301" > "30130"
    vec.sort_by(|a, b| {
        let mut a = a.clone();
        let mut b = b.clone();
        a.push_str(&b);
        b.push_str(&a);
        b.cmp(&a)
    });

    vec.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_01() {
        assert!("A" < "a");
        assert!("Aa" < "a");
        assert!("301" > "30");
    }

    #[test]
    fn t_slice_to_max_num() {
        assert_eq!(slice_to_max_num(&[1, 4, 30, 34, 301, 9, 5]), "95434303011".to_string());
    }
}
