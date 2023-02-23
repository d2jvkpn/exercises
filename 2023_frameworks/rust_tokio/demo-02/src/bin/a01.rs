use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initial input
    let mut v = "Hello, ".to_string();

    let res = task::spawn_blocking(move || {
        // Stand-in for compute-heavy work or using synchronous APIs
        v.push_str("world");
        // Pass ownership of the value back to the asynchronous context
        v
    })
    .await?;

    // `res` is the value returned from the thread
    assert_eq!(res.as_str(), "Hello, world");

    Ok(())
}
