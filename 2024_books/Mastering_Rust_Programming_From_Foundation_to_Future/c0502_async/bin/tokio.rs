use tokio::{fs, io::AsyncWriteExt};

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    fs::create_dir_all("data").await?;

    let mut file = fs::File::create("data/tokio_a01.txt").await?;
    file.write_all(b"Hello, Tokio World!\n").await?;

    let contents = fs::read_to_string("data/tokio_a01.txt").await?;
    println!("~~~ contents: {:?}", contents);

    match fetch_data("http://127.0.0.1:8000").await {
        Err(e) => return Err(io::Error::other(format!("fetch_data: {}", e))),
        Ok(v) => println!("~~~ fetch_data: {}", v),
    }

    Ok(())
}

// $ python3 -m http.server
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    response.text().await
}
