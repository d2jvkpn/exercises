use chrono::{Local, SecondsFormat};
use tokio::{
    select, spawn,
    sync::{broadcast, mpsc},
    time,
};

use std::time::Duration;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(1);
    let (tx2, mut rx2) = broadcast::channel(1);

    spawn(async move {
        for i in 0..10 {
            select! {
                result = rx2.recv() => {
                    println!("--> {} Got broadcast: {result:?}", now());
                    return;
                }
                result = tx1.send(i) => {
                    if let Err(e) = result {
                        eprintln!("!!! {} send error: {e:?}", now());
                        return;
                    }
                }
            };

            println!("--> {} send: {i}", now());
            time::sleep(Duration::new(1, 0)).await;
        }
    });

    let t1 = spawn(async move {
        let mut n = 0;
        while let Some(v) = rx1.recv().await {
            n += 1;
            println!("<-- {} got: {v}", now());
        }
        n
    });

    time::sleep(Duration::new(5, 0)).await;
    tx2.send(false).unwrap();

    println!("==> Received {} items", t1.await.unwrap());
}

fn now() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
