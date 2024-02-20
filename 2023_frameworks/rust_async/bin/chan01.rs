use tokio::{spawn, sync::mpsc};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1);

    let tx_clone = tx.clone();
    spawn(async move {
        for i in 0..10 {
            if let Err(e) = tx_clone.send(i).await {
                eprintln!("!!! send error: {e:?}");
            }
            println!("--> send: {i}");
        }
    });

    let tx_clone = tx.clone();
    spawn(async move {
        for i in 10..20 {
            if let Err(e) = tx_clone.send(i).await {
                eprintln!("!!! send error: {e:?}");
            }
            println!("--> send: {i}");
        }
    });

    drop(tx);

    let mut n = 0;
    while let Some(v) = rx.recv().await {
        n += 1;
        println!("<-- got: {v}");
    }

    println!("==> Received {n} items");
}
