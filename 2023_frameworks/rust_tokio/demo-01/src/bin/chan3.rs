use chrono::{Local, SecondsFormat};
use std::time::Duration;
use tokio::sync::{mpsc::channel, oneshot};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = channel(1);
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async move {
        for i in 0..5 {
            match tx.send(i).await {
                Ok(_) => println!("--> {} send: {i}", now()),
                Err(e) => {
                    eprintln!("!!! {} send error: {e:?}", now());
                    break;
                }
            }

            tokio::time::sleep(Duration::new(1, 0)).await;
        }
        println!("=== {} finished send", now());
        tx2.send(true).unwrap();
    });

    let t1 = tokio::spawn(async move {
        let mut n = 0;
        while let Some(v) = rx.recv().await {
            n += 1;
            println!("<-- {} got: {v}", now());
        }
        println!("=== {} ended recv", now());
        n
    });

    rx2.await.unwrap();
    println!("==> Received {} items", t1.await.unwrap());
}

fn now() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
