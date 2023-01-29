use serde_json::{json, value::Value, Map};
use std::{fs, io::Read};

pub fn read_file() -> Map<String, Value> {
    let mut file = fs::File::open(crate::get_state_filepath()).unwrap();

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let val: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = val.as_object().unwrap().clone();

    return state;
}

pub fn save_to_file(state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(crate::get_state_filepath(), new_data.to_string()).expect("Unable to write file");
}
