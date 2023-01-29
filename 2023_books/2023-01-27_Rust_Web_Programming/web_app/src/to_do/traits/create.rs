use serde_json::{json, value::Value, Map};

use crate::state::save_to_file;

pub trait Create {
    fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        save_to_file(state);
        println!(">>> Item: {} is being created", title);
    }
}
