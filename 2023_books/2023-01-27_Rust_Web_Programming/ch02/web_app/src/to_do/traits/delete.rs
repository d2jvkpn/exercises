use serde_json::{value::Value, Map};

use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file(crate::get_state(), state);
        println!(">>> Item: {} is being deleted", title);
    }
}
