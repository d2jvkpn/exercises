use futures::{stream, StreamExt};
use reqwest::Client;
use tokio;

const CONCURRENT_REQUESTS: usize = 2;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let urls = vec!["https://api.ipify.org"; 2];

    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let resp = client.get(url).send().await?;
                resp.bytes().await
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => println!("Got {} bytes", b.len()),
                Err(e) => eprintln!("Got an error: {}", e),
            }
        })
        .await;
}
