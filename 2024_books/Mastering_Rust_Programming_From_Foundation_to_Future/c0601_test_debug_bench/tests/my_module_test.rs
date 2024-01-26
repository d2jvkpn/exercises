#[cfg(test)]
mod tests {
    use c0601::my_module;

    #[test]
    fn t_add() {
        dbg!(file!());

        assert_eq!(my_module::add(2, 2), 4);
        assert_eq!(my_module::add(1, -1), 0);
        assert_eq!(my_module::add(0, 0), 0);
    }
}
