#![allow(dead_code, unused_variables)]

use std::{cell::RefCell, rc::Rc};

fn main() {
    // println!("Hello, wrold!");

    let db = Rc::new(RefCell::new(Database { max_connections: 100 }));

    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };

    dbg!(&content_service);

    db.borrow_mut().max_connections = 200;

    dbg!(&content_service);
}

#[derive(Debug)]
struct Database {
    max_connections: u32,
}

#[derive(Debug)]
struct AuthService {
    db: Rc<RefCell<Database>>,
}

#[derive(Debug)]
struct ContentService {
    db: Rc<RefCell<Database>>,
}
