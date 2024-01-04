use reqwest;
use serde::Deserialize;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let data = fetch_data("hello", 3);
    let time_since = calculate_last_login();

    let (posts, _) = tokio::join!(data, time_since);
    println!("~~~ Fetched {:?}", posts);

    Ok(())
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Response {
    url: String,
    origin: String,
    args: serde_json::Value,
}

async fn fetch_data(name: &str, seconds: u64) -> Result<Response, reqwest::Error> {
    let request_url = format!("https://httpbin.org/delay/{}?name={}", seconds, name);
    println!("==> Fetching: {}", request_url);

    let response = reqwest::get(request_url).await?;
    let response: Response = response.json().await?;

    Ok(response)
}

async fn calculate_last_login() {
    sleep(Duration::from_secs(1)).await;
    println!("==> Logged in 2 days ago");
}
