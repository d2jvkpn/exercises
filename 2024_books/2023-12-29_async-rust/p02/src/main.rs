#![allow(unused_imports)]

use std::{
    fs::{create_dir_all, File, OpenOptions},
    future::Future,
    io::prelude::*,
    path::Path,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use chrono::{Local, SecondsFormat};
use futures_util::future::join_all;
use tokio::task::{spawn, JoinHandle};

type AsyncFileHandle = Arc<Mutex<File>>;
// type FileJoinHandle = JoinHandle<Result<bool, String>>;

#[tokio::main]
async fn main() {
    // println!("Hello, world!");

    let login_handle = get_handle(&"logs/login.txt");
    let logout_handle = get_handle(&"logs/logout.txt");

    let names = ["one", "two", "three", "four", "five", "six"];

    let mut handles = Vec::new();

    for name in names {
        let handle1 = login_handle.clone();
        let handle2 = logout_handle.clone();

        let handle1 = write_log(handle1, name.to_string());
        let handle2 = write_log(handle2, name.to_string());

        handles.push(handle1);
        handles.push(handle2);
    }

    let _ = join_all(handles).await;
}

fn get_handle(fp: &str) -> AsyncFileHandle {
    match OpenOptions::new().append(true).open(fp) {
        Ok(v) => return Arc::new(Mutex::new(v)),
        Err(_) => {}
    }

    let path = Path::new(fp).parent().unwrap();
    create_dir_all(path).unwrap();

    return Arc::new(Mutex::new(File::create(fp).unwrap()));
}

struct AsyncWriteFuture {
    pub handle: AsyncFileHandle,
    pub entry: String,
}

impl Future for AsyncWriteFuture {
    type Output = Result<bool, String>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let mut guard = match self.handle.try_lock() {
            Ok(v) => v,
            Err(e) => {
                println!("!!! error: {}", e);
                ctx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        let lined_entry = format!(
            "{} {}\n",
            Local::now().to_rfc3339_opts(SecondsFormat::Millis, false),
            self.entry,
        );

        match guard.write_all(lined_entry.as_bytes()) {
            Ok(_) => print!("~~~ written for: {}", lined_entry),
            Err(e) => {
                println!("!!! {}", e);
                ctx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        Poll::Ready(Ok(true))
    }
}

async fn write_log(file_handle: AsyncFileHandle, line: String) {
    let future = AsyncWriteFuture {
        handle: file_handle,
        entry: line,
    };

    // spawn(async move { future.await });
    let _ = future.await;
}
