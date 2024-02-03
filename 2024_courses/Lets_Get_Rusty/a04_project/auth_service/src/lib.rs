#![allow(unused_variables, dead_code)]
mod help;
mod x001;

use rand::prelude::*;

use crate::x001::{b01, multiply};

mod database {
    #[derive(PartialEq)]
    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        Status::Connected
    }

    pub fn get_user(username: impl AsRef<str>) {}
}

pub mod auth {
    use super::*;

    pub struct Credentials {
        pub username: String,
        pub password: String,
    }

    pub fn login(creds: &Credentials) {
        database::get_user(&creds.username);
        help::do_nothing();

        let (x, y) = (2, 2);

        println!(
            "{x}+{y}={ans1}, {x}*{y}={ans2}, {x}/{y}={ans3}",
            x = x,
            y = y,
            ans1 = utils::add(x, y),
            ans2 = multiply(x, y),    // crate::x001::multiply
            ans3 = b01::divide(x, y), // crate::x001::b01::divide
        );
    }

    fn logout(username: impl AsRef<str>) {}
}

pub fn authenticate(creds: &auth::Credentials) {
    let timeout = thread_rng().gen_range(100..500);
    println!("==> The timeout is {timeout}.");

    if database::Status::Connected == database::connect_to_database() {
        auth::login(creds);
    }
}

/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
