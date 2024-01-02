use std::{
    fs::{File, OpenOptions},
    future::Future,
    io::prelude::*,
    path::Path,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use futures_util::future::join_all;
use tokio::task::{JoinHandle, spawn};

type AsyncFileHandle = Arc<Mutex<File>>;
type FileJoinHandle = JoinHandle<Result<bool, String>>;

#[tokio::main]
async fn main() {
    // println!("Hello, world!");

	let login_handle = get_handle(&"login.txt");
	let logout_handle = get_handle(&"logout.txt");

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
        Ok(v) => Arc::new(Mutex::new(v)),
        Err(_) => Arc::new(Mutex::new(File::create(fp).unwrap())),
    }
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

        let lined_entry = format!("{}\n", self.entry);

        match guard.write_all(lined_entry.as_bytes()) {
            Ok(_) => println!("~~~ written for: {}", lined_entry),
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
	let future = AsyncWriteFuture{
		handle: file_handle,
		entry: line,
	};

	spawn(async move {
		future.await
	});
}
