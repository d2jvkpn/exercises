#![allow(dead_code)]

fn main() {
    let alice = User::new("Alice", "abc");
    let bob = Admin::new("Bob", "abc");

    alice.edit();
    bob.edit();

    let vec: Vec<&dyn CanEdit> = vec![&alice, &bob];
    vec.iter().for_each(|v| v.edit());
}

struct Admin {
    username: String,
    password: String,
}

impl Admin {
    fn new(username: &str, password: &str) -> Self {
        let (username, password) = (username.into(), password.into());
        Self { username, password }
    }
}

struct User {
    username: String,
    password: String,
}

impl User {
    fn new(username: &str, password: &str) -> Self {
        let (username, password) = (username.into(), password.into());
        Self { username, password }
    }

    fn edit(&self) {
        println!("USER is editing");
    }
}

trait CanEdit {
    fn edit(&self) {
        println!("Editing implemented by CanEdit");
    }
}

impl CanEdit for Admin {}

impl CanEdit for User {
    fn edit(&self) {
        println!("User {:?} is editing", self.username);
    }
}
