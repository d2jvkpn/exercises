use serde_json::{value::Value, Map};

use super::to_do::{
    structs::{Done, Pending},
    traits::{Create, Delete, Edit, Get},
    ItemTypes,
};

fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "create" => {
            item.create(&item.super_struct.title, &item.super_struct.status.stringify(), &mut state)
        }
        "edit" => item.set_to_done(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command),
    }
}

fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command),
    }
}

pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state),
    }
}
