#![allow(dead_code)]

use std::{any, time};

fn type_name<T>(_v: &T) -> String {
    format!("{}", any::type_name::<T>())
}

fn elapsed(start: time::Instant) -> time::Duration {
    time::Duration::from_millis(start.elapsed().as_millis().try_into().unwrap())
}
