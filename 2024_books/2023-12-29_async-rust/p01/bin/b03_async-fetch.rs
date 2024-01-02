use chrono::Local;
use tokio::{
    fs::File as AsyncFile,
    io::{self, AsyncReadExt},
    sync::watch,
    time::{sleep, Duration},
};

use std::path::PathBuf;

#[tokio::main]
async fn main() {
    // println!("Hello, wrold!");

    let fp = "data/data.txt";
    let (tx, mut rx) = watch::channel(false);

    tokio::spawn(watch_file_changes(fp, tx));

    loop {
        let _ = rx.changed().await;

        if let Ok(contents) = read_file(fp).await {
            println!("==> {:?} file updated: {}\n{}", Local::now(), fp, contents.trim());
        }
    }
}

async fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = AsyncFile::open(filename).await?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

async fn watch_file_changes(fp: &str, tx: watch::Sender<bool>) {
    let path = PathBuf::from(fp);
    let mut last_modified = None;

    loop {
        if let Ok(metadata) = path.metadata() {
            let modified = metadata.modified().unwrap();
            if last_modified != Some(modified) {
                last_modified = Some(modified);
                let _ = tx.send(true);
            }
        }

        sleep(Duration::from_millis(100)).await;
    }
}
