use crate::HelloWorld;

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
