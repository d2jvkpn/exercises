use std::cmp::Ordering;

fn median(vec: &mut Vec<f32>) -> Option<f32> {
    if vec.is_empty() {
        return None;
    }

    vec.sort_by(|a: &f32, b: &f32| a.partial_cmp(b).unwrap_or(Ordering::Less));

    let n = vec.len();

    let ans = if n % 2 == 0 {
        (vec[n / 2] + vec[n / 2 - 1]) / 2.0
    } else {
        vec[n / 2]
    };

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_median() {
        let mut vec = vec![1.0, 4.0, 5.0];
        assert_eq!(median(&mut vec), Some(4.0));

        let mut vec = vec![1.0, 3.0, 4.0, 5.0];
        assert_eq!(median(&mut vec), Some(3.5));
    }
}
