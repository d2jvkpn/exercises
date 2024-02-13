#![allow(dead_code)]

#[derive(Debug)]
struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    pub fn new(name: impl AsRef<str>, payload: T) -> Self {
        Self { name: name.as_ref().to_string(), payload }
    }

    pub fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    pub fn print_payload(&self) {
        println!("~~~ {} payload: {}", self.name, self.payload);
    }
}

fn main() {
    let cmd1 = BrowserCommand {
        name: "navigate".to_owned(),
        payload: "https://letsgetrusty.com".to_owned(),
    };
    dbg!(&cmd1);

    cmd1.print_payload();

    // let cmd2 = BrowserCommand::<usize>::new("zoom", 200);
    let cmd2 = BrowserCommand::new("zoom", 200);
    dbg!(&cmd2);

    println!("~~~ get_payload: {}", cmd2.get_payload());

    println!("cmd1 serialize_payload: {}", serialize_payload(cmd1));
    println!("cmd2 serialize_payload: {}", serialize_payload(cmd2));
}

fn serialize_payload<T>(_payload: T) -> String {
    "placeholder".to_owned()
}
