#![allow(dead_code)]

#[macro_export]
macro_rules! code_loc {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        // let caller = std::panic::Location::caller();
        let name = type_name_of(f);
        let list: Vec<&str> = name.split("::").collect();
        // println!("??? {:?}", list);
        let length = list.len();
        let idx = if list[length - 2] == "{{closure}}" {
            length - 3
        } else {
            length - 2
        };

        format!("{}:{}:{}", file!(), line!(), list[idx])
    }};
}

use anyhow::Context;
use std::{error::Error, fs::File};

#[derive(Debug, derive_more::Display, derive_more::Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

#[derive(Debug, derive_more::Display)]
enum E2 {
    #[display(fmt = "internal error: {}", .0)]
    Internal(anyhow::Error),
}

fn call1() -> Result<(), MyError> {
    Err(MyError::InternalError)
}

fn call2() -> anyhow::Result<()> {
    Err(MyError::InternalError).context(code_loc!())
}

fn main() {
    if let Err(e) = call1() {
        println!("!!! my error: {}, {:?}", e, e.source());
    }

    if let Err(e) = call2() {
        println!("!!! my error: {:?}, {:?}", e, e.source());
    }

    if let Err(e) = File::open("not_exists.txt").context(code_loc!()) {
        println!("{}", e);
    }
}
