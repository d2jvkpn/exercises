use serde_json::{value::Value, Map};

use super::to_do::{
    structs::{Done, Pending},
    traits::{Create, Delete, Edit, Get},
    ItemTypes,
};

fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    let title = &item.super_struct.title;

    match command.as_str() {
        "get" => item.get(title, &state),
        "create" => item.create(title, item.super_struct.status_str(), &mut state),
        "edit" => item.set_to_done(title, &mut state),
        _ => println!("command: {command} not supported"),
    }
}

fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    let title = &item.super_struct.title;

    match command.as_str() {
        "get" => item.get(title, &state),
        "delete" => item.delete(title, &mut state),
        "edit" => item.set_to_pending(title, &mut state),
        _ => println!("command: {command} not supported"),
    }
}

pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state),
    }
}
