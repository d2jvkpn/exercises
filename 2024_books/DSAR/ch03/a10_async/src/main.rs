use futures::future::join_all;
use reqwest;
use tokio::task;

use std::{env, error::Error, time::Duration};

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://www.rust-lang.org",
        "https://www.mozilla.org",
        "https://www.github.com",
        "https://not_exists.local",
    ];

    /*
    let mut handles = vec![];

    for url in urls {
        let handle = task::spawn(fetch_url(url.to_string()));
        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(content) => println!("Content fetched: {} bytes", content.len()),
            Err(e) => eprintln!("Failed to fetch content: {}", e),
        }
    }
    */

    // convert Result<Result<String, reqwest::Error>, tokio::task::JoinError>
    // let results: Vec<_> = join_all(handles).await;

    /*
    |r| -> Result<String, Box<dyn Error>> {
        match r {
        Ok(v) => v.map_err(Into::into),
        Err(e) => Err(e.into()),
        }
    }
    */

    let handles = urls.iter().map(|url| task::spawn(fetch_url(url)));

    let results: Vec<Result<String, Box<dyn Error>>> =
        join_all(handles).await.into_iter().map(|r| Ok(r??)).collect();

    println!("==> results: {:?}", results);
}

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    // let res = reqwest::get(&url).await.expect("Failed to fetch URL");
    // res.text().await.expect("Failed to read response")

    let mut builder = reqwest::Client::builder().timeout(Duration::new(5, 0));

    let proxy_url = env::var("https_proxy").unwrap_or("".to_string());

    if proxy_url.len() > 0 {
        dbg!(&proxy_url);
        let proxy = reqwest::Proxy::all(&proxy_url)?;
        builder = builder.proxy(proxy);
    }

    let client: reqwest::Client = builder.build()?;

    let res = client.get(url).send().await?;
    let text = res.text().await?;

    Ok(format!("Content fetched from {url}: {} bytes", text.len()))
}
