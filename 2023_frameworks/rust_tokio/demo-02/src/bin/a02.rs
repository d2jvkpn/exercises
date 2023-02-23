use tokio::{sync::mpsc, task};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel(2);
    let start = 5;

    let worker = task::spawn_blocking(move || {
        for x in 0..10 {
            // Stand in for complex computation
            tx.blocking_send(start + x).unwrap();
        }
    });

    let mut acc = 0;
    while let Some(v) = rx.recv().await {
        acc += v;
    }

    assert_eq!(acc, 95);
    worker.await.unwrap();

    Ok(())
}
