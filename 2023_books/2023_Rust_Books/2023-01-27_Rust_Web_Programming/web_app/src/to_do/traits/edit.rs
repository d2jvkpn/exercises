use serde_json::{json, value::Value, Map};

use super::super::enums::TaskStatus::{Done, Pending};
use crate::state::save_to_file;

pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(Done.as_str()));
        save_to_file(state);
        println!(">>> Item: {title} is being set to done");
    }

    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(Pending.as_str()));
        save_to_file(state);
        println!(">>> Item: {title} is being set to pending");
    }
}
