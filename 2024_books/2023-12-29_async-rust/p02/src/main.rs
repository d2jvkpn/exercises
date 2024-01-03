#![allow(unused_imports)]

use std::{
    error::Error,
    fs::{create_dir_all, File, OpenOptions},
    future::Future,
    io::{self, prelude::*},
    path::Path,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use chrono::{Local, SecondsFormat};
use futures_util::future::join_all;
use tokio::{
    sync::oneshot,
    task::{spawn, JoinHandle},
};

type AsyncFileHandle = Arc<Mutex<File>>;
// type FileJoinHandle = JoinHandle<Result<bool, String>>;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // println!("Hello, world!");

    let login_handle = get_handle("logs/login.txt").unwrap();
    let logout_handle = get_handle("logs/logout.txt").unwrap();

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

fn get_handle(fp: &str) -> Result<AsyncFileHandle, io::Error> {
    let path = Path::new(fp).parent().unwrap(); // TODO
    create_dir_all(path)?;

    OpenOptions::new().append(true).create(true).open(fp).and_then(|v| Ok(Arc::new(Mutex::new(v))))

    // let file = File::create(fp)?;
    // Ok(Arc::new(Mutex::new(file)))
}

struct AsyncWriteFuture {
    pub handle: AsyncFileHandle,
    pub entry: String,
}

fn now_rfc3339() -> String {
    // Local::now().to_rfc3339_opts(SecondsFormat::Millis, false)
    Local::now().to_rfc3339_opts(SecondsFormat::Nanos, false)
}

impl Future for AsyncWriteFuture {
    type Output = Result<bool, String>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let now = now_rfc3339();

        let mut guard = match self.handle.try_lock() {
            Ok(v) => v,
            Err(e) => {
                println!("!!! {} error: {}", now, e);
                ctx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        let lined_entry = format!("{} {}\n", now, self.entry);

        match guard.write_all(lined_entry.as_bytes()) {
            Ok(_) => print!("~~~ {} written: {}\n", now, self.entry),
            Err(e) => {
                println!("!!! {} write error: {}", now, e);
                ctx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        Poll::Ready(Ok(true))
    }
}

async fn write_log(file_handle: AsyncFileHandle, line: String) {
    let future = AsyncWriteFuture { handle: file_handle, entry: line };

    // a:
    // spawn(async move { future.await }); // out of control

    // b:
    // let _ = future.await;

    // c:
    let (tx, rx) = oneshot::channel();

    spawn(async move {
        let _ = future.await;
        let _ = tx.send(true);
    });

    let _ = rx.await;
}
