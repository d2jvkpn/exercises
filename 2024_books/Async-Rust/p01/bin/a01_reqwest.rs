use std::time::Instant;

use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // println!("Hello, world!");

    let url = "https://www.baidu.com";
    let start_time = Instant::now();

    let ans = reqwest::get(url).await?;

    let elapsed = start_time.elapsed();
    println!("==> response: {}, request took {}ms.", ans.status(), elapsed.as_millis());

    Ok(())
}
