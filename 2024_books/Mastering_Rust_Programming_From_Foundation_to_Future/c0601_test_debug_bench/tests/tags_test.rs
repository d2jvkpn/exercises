#[test]
#[ignore]
fn t_ignored() {
    // This test will be ignored when running 'cargo test'.
    // Useful for temporarily disabling tests.
}

#[test]
#[should_panic]
fn t_panic() {
    // This test is expected to panic during execution.
    panic!("This test should panic!");
}

#[cfg(test)]
mod conditional_tests {
    #[cfg(feature = "feature_x")]
    #[test]
    fn t_feature_x() { // This test is only compiled and run if the "feature_x" feature is enabled.
    }

    #[cfg(not(feature = "feature_x"))]
    #[test]
    fn t_without_feature_x() {
        // This test is only compiled and run if the "feature_x" feature is disabled.
    }
}
