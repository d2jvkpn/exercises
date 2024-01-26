pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn t_add() {
        dbg!(file!());

        assert_eq!(add(2, 2), 4);
        assert_eq!(add(1, -1), 0);
        assert_eq!(add(0, 0), 0);
    }
}
