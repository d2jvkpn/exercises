use async_std::{fs, io::prelude::*, task};

use std::io;

/*
fn main() {
    // println!("Hello, wrold!");

    task::block_on(async {
        write_to_file().await.unwrap();
        read_file().await.unwrap();
    });
}
*/

#[async_std::main]
async fn main() -> io::Result<()> {
    write_to_file().await?;
    read_file().await?;

    Ok(())
}

async fn write_to_file() -> io::Result<()> {
    fs::create_dir_all("data").await?;

    let mut file = fs::File::create("data/async_a01.txt").await?;

    file.write_all(b"Hello, Async World!\n").await?;

    Ok(())
}

async fn read_file() -> io::Result<()> {
    let contents = fs::read_to_string("data/async_a01.txt").await?;
    println!("~~~ contens: {:?}", contents);

    Ok(())
}
