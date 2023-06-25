fn is_prime(num: usize) -> Option<bool> {
    if num < 2 {
        return None;
    }

    let r = (num as f64).sqrt() as usize;
    for v in 2..r {
        if num % v == 0 {
            return Some(false);
        }
    }

    Some(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_is_prime() {
        assert_eq!(is_prime(2), Some(true));
        assert_eq!(is_prime(2023), Some(false));
    }
}
