pub fn replace_option<T>(a: &mut Option<T>, b: &mut Option<T>) {
    if b.is_none() {
        return;
    }
    *a = b.take();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_replace_option() {
        let mut a = Some("A");
        let mut b = Some("B");

        replace_option(&mut a, &mut b);
        assert_eq!(a, Some("B"));
        assert_eq!(b, None);

        let mut a = Some("A");
        let mut b = None;
        replace_option(&mut a, &mut b);
        assert_eq!(a, Some("A"));
        assert_eq!(b, None);

        let mut a = None;
        let mut b = Some("B");
        replace_option(&mut a, &mut b);
        assert_eq!(a, Some("B"));
        assert_eq!(b, None);
    }
}
