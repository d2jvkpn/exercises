use serde_json::{value::Value, Map};

use crate::state::save_to_file;

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        save_to_file(state);
        println!(">>> Item: {title} is being deleted");
    }
}
