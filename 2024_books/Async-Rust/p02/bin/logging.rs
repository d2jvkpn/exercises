#![allow(dead_code)]

use chrono::{Local, SecondsFormat};
use tokio::{sync::oneshot, task::spawn};

use std::{
    error,
    fs::{create_dir_all, File, OpenOptions},
    future::Future,
    io::{self, prelude::*},
    path::Path,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

// type FileJoinHandle = JoinHandle<Result<bool, String>>;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn error::Error>> {
    // println!("Hello, world!");

    let login = Logger::new("logs/login.txt")?;
    let logout = Logger::new("logs/logout.txt")?;

    let names = ["one", "two", "three", "four", "five", "six"];

    for name in names {
        login.write(name.to_string()).await;
        logout.write(name.to_string()).await;
    }

    Ok(())
}

struct Entry {
    pub handle: Arc<Mutex<File>>,
    pub line: String,
}

struct Logger {
    pub fp: String,
    pub handle: Arc<Mutex<File>>,
}

impl Logger {
    fn new(fp: &str) -> Result<Self, io::Error> {
        let path = Path::new(fp).parent().ok_or(io::ErrorKind::Other)?;
        create_dir_all(path)?;

        let file = OpenOptions::new().append(true).create(true).open(fp)?;

        // let file = File::create(fp)?;
        // Ok(Arc::new(Mutex::new(file)))
        Ok(Self { fp: fp.to_string(), handle: Arc::new(Mutex::new(file)) })
    }

    async fn write(&self, line: String) {
        let future = Entry { handle: self.handle.clone(), line };

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
}

impl Future for Entry {
    type Output = Result<bool, String>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        // Local::now().to_rfc3339_opts(SecondsFormat::Millis, false)
        let now = Local::now().to_rfc3339_opts(SecondsFormat::Nanos, false);

        let mut guard = match self.handle.try_lock() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("!!! {} ERROR: {}", now, e);
                ctx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        let line = format!("{} {}\n", now, self.line);

        match guard.write_all(line.as_bytes()) {
            Ok(_) => { /*println!("~~~ {} written: {}", now, self.line)*/ }
            Err(e) => {
                eprintln!("!!! {} write: {}", now, e);
                ctx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        Poll::Ready(Ok(true))
    }
}
