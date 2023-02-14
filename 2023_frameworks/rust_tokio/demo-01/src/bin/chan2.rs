use chrono::{Local, SecondsFormat};
use std::time::Duration;
use tokio::sync::{broadcast, mpsc::channel};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = channel(1);
    let (tx2, mut rx2) = broadcast::channel(1);

    tokio::spawn(async move {
        for i in 0..10 {
            tokio::select! {
                result = rx2.recv() => {
                    println!("--> {} Got broadcast: {result:?}", now());
                    return;
                }
                result = tx.send(i) => {
                    if let Err(e) = result {
                        eprintln!("!!! {} send error: {e:?}", now());
                        return;
                    }
                }
            };

            println!("--> {} send: {i}", now());
            tokio::time::sleep(Duration::new(1, 0)).await;
        }
    });

    let t1 = tokio::spawn(async move {
        let mut n = 0;
        while let Some(v) = rx.recv().await {
            n += 1;
            println!("<-- {} got: {v}", now());
        }
        n
    });

    tokio::time::sleep(Duration::new(5, 0)).await;
    tx2.send(false).unwrap();

    println!("==> Received {} items", t1.await.unwrap());
}

fn now() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
