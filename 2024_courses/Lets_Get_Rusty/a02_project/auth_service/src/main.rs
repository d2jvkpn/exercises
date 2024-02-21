use auth_service::{auth, authenticate};

fn main() {
    let creds =
        auth::Credentials { username: "letsgetrusty".to_owned(), password: "123456".to_owned() };

    authenticate(&creds);
}
