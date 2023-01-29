use serde_json::{json, value::Value, Map};

use super::super::enums::TaskStatus::{DONE, PENDING};
use crate::state::write_to_file;

pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(DONE.stringify()));
        write_to_file(crate::get_state(), state);
        println!(">>> Item: {} is being set to done", title);
    }

    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(PENDING.stringify()));
        write_to_file(crate::get_state(), state);
        println!(">>> Item: {} is being set to pending", title);
    }
}
