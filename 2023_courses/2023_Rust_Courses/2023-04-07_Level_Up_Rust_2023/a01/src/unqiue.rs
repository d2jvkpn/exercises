fn unique_01(vec: &mut Vec<f32>) -> Vec<f32> {
    let mut ans = Vec::new();

    for e in vec {
        if !ans.contains(e) {
            ans.push(*e)
        }
    }

    ans
}

fn unique_02<T: Ord>(mut vec: Vec<T>) -> Vec<T> {
    // vec.sort();
    vec.sort_by(|x, y| x.cmp(y));
    vec.dedup();
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_unique() {
        let mut vec = vec![1.0, 3.0, 2.0, 1.9, 2.0];
        assert_eq!(unique_01(&mut vec), vec![1.0, 3.0, 2.0, 1.9]);

        let mut vec = vec![];
        assert!(unique_01(&mut vec).is_empty());

        let vec = vec![1, 9, 6, 5];
        assert_eq!(unique_02(vec), vec![1, 5, 6, 9]);
    }
}
