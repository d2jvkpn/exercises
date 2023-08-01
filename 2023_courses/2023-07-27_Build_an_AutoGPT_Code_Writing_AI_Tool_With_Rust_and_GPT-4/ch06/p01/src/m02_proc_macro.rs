#![allow(dead_code)]

use proc_macro::HelloWorld;

/*
// Serialize, Deserialize
#[derive(Debug, Clone, PartialEq, Default)]
struct Account {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
*/

#[derive(HelloWorld)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_hello_world() {
        let user = User { id: 1, name: "Rover".into(), email: "rover@local.local".into() };
        user.hello_world();
    }
}
