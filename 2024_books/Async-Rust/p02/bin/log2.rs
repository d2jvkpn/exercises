use chrono::{Local, SecondsFormat};
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::AsyncWriteExt,
};

use std::{error, io, path::Path};

// type FileJoinHandle = JoinHandle<Result<bool, String>>;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn error::Error>> {
    // println!("Hello, world!");

    let mut login = Logger::new("logs/login.txt").await?;
    let mut logout = Logger::new("logs/logout.txt").await?;

    let names = ["one", "two", "three", "four", "five", "six"];

    for name in names {
        login.write(name.to_string()).await?;
        logout.write(name.to_string()).await?;
    }

    Ok(())
}

struct Logger {
    pub handle: File,
}

impl Logger {
    async fn new(fp: &str) -> Result<Self, io::Error> {
        let path = Path::new(fp).parent().ok_or(io::ErrorKind::Other)?;
        create_dir_all(path).await?;

        let file = OpenOptions::new().append(true).create(true).open(fp).await?;

        // let file = File::create(fp)?;
        // Ok(Arc::new(Mutex::new(file)))
        Ok(Self { handle: file })
    }

    async fn write(&mut self, line: String) -> Result<(), io::Error> {
        // Local::now().to_rfc3339_opts(SecondsFormat::Millis, false)
        let now = Local::now().to_rfc3339_opts(SecondsFormat::Nanos, false);
        let line = format!("{} {}\n", now, line);

        self.handle.write_all(line.as_bytes()).await
    }
}
