use serde_json::Value;

async fn my_async_call(url: &str) -> Result<Value, reqwest::Error> {
    let response = reqwest::get(url).await?.json::<Value>().await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn t_my_async_call() {
        let api_url = "https://cat-fact.herokuapp.com/facts/";
        let response = my_async_call(api_url).await.unwrap();
        println!("response: {:?}", response);
    }
}
