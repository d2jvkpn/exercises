use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("==> http_get");
    http_get().await?;

    println!("==> http_get_sequential");
    http_get_sequential().await?;

    println!("==> http_get_async");
    http_get_async().await?;

    Ok(())
}

async fn http_get() -> Result<(), reqwest::Error> {
    let url = "https://www.baidu.com";
    let start_time = Instant::now();

    let ans = reqwest::get(url).await?;

    let elapsed = start_time.elapsed();
    println!("--> response: {}, request took {}ms.", ans.status(), elapsed.as_millis());

    Ok(())
}

async fn http_get_sequential() -> Result<(), reqwest::Error> {
    let url = "https://www.baidu.com";
    let start_time = Instant::now();

    let _ = reqwest::get(url).await?;
    let _ = reqwest::get(url).await?;
    let _ = reqwest::get(url).await?;
    let _ = reqwest::get(url).await?;
    let ans = reqwest::get(url).await?;

    let elapsed = start_time.elapsed();

    println!("--> response: {}, request took {}ms.", ans.status(), elapsed.as_millis());

    Ok(())
}

async fn http_get_async() -> Result<(), reqwest::Error> {
    let url = "https://www.baidu.com";
    let start_time = Instant::now();

    let (_, _, _, _, ans) = tokio::join!(
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url)
    );

    let ans = ans?;
    let elapsed = start_time.elapsed();

    println!("--> response: {}, request took {}ms.", ans.status(), elapsed.as_millis());

    Ok(())
}
