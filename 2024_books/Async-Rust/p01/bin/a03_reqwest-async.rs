use std::time::Instant;

use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // println!("Hello, world!");

    let url = "https://www.baidu.com";
    let start_time = Instant::now();

    let (_, _, _, ans) =
        tokio::join!(reqwest::get(url), reqwest::get(url), reqwest::get(url), reqwest::get(url));

    let ans = ans?;
    let elapsed = start_time.elapsed();

    println!("==> response: {}, request took {}ms.", ans.status(), elapsed.as_millis());

    Ok(())
}
