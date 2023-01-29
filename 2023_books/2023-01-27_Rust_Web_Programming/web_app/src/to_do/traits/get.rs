use serde_json::{value::Value, Map};

pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);

        match item {
            Some(v) => println!(">>> Item: {}, Status: {}", title, v),
            None => println!("Item: {} was not found", title),
        }
    }
}
