#![allow(dead_code)]

fn main() {
    // println!("Hello, wrold!");

    /*
    let validator =
        |creds: &Credentials| -> bool { !creds.username.is_empty() && !creds.password.is_empty() };
    */

    let weak_password = "password123!".to_owned();
    let validator1 = move |username: &str, password: &str| -> bool {
        !username.is_empty()
            && !password.len() >= 8
            && password.contains(['!', '@', '#', '$', '%', '^', '&', '*', '(', ')'])
            && password != weak_password
    };

    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password123!".to_owned(),
        validator: validator1,
    };

    println!(
        "~~~ Valid Credentials: {}, {}",
        creds.is_valid(),
        get_password_validator1(8, true)(&creds.username, &creds.password),
    );
}

struct Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    username: String,
    password: String,
    validator: T,
}

impl<T> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    pub fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

// static dispatch
fn get_password_validator1(min_len: usize, special_char: bool) -> impl Fn(&str, &str) -> bool {
    move |username: &str, password: &str| {
        !username.is_empty()
            && !password.len() >= min_len
            && (special_char
                && password.contains(['!', '@', '#', '$', '%', '^', '&', '*', '(', ')']))
            && password != "password123!"
    }
}

// dynamic dispatch
fn get_password_validator2(min_len: usize, special_char: bool) -> Box<dyn Fn(&str, &str) -> bool> {
    if special_char {
        let validator = move |username: &str, password: &str| {
            !username.is_empty()
                && !password.len() >= min_len
                && (special_char
                    && password.contains(['!', '@', '#', '$', '%', '^', '&', '*', '(', ')']))
                && password != "password123!"
        };
        Box::new(validator)
    } else {
        let validator = move |username: &str, password: &str| {
            !username.is_empty() && !password.len() >= min_len
        };
        Box::new(validator)
    }
}

impl<T> Credentials<T>
where
    T: Fn(&str, &str) -> bool,
{
    pub fn new(username: impl AsRef<str>, password: impl AsRef<str>, validator: T) -> Self {
        Self {
            username: username.as_ref().to_string(),
            password: password.as_ref().to_string(),
            validator,
        }
    }
}
