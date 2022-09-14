use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Local, SecondsFormat};
use futures::future::*;
use tokio::spawn;
use tokio::sync::{broadcast, mpsc, oneshot, Mutex};
use tokio::time::sleep;

fn now() -> String {
    let now: DateTime<Local> = Local::now();
    return now.to_rfc3339_opts(SecondsFormat::Millis, true);
}

async fn some_computation(input: u32) -> String {
    sleep(Duration::from_millis(500)).await;
    // "represents the result of the computation".to_string()

    format!("the result of computation {}", input)
}

#[tokio::main]
async fn main() {
    // onshot
    let (tx, rx) = oneshot::channel();

    spawn(async move {
        let res = some_computation(42).await;
        tx.send(res).unwrap();
    });
    // Do other work while the computation is happening in the background
    // Wait for the computation result

    let res = rx.await.unwrap();
    println!("~~~ oneshot result: {}", res);

    // mpsc
    let (tx, mut rx) = mpsc::channel(4);
    //    tokio::spawn(async move {
    //        for i in 0..16 {
    //            let res = some_computation(i).await;
    //            tx.send(res).await.unwrap();
    //        }
    //    });
    for i in 0..16 {
        let txc = tx.clone(); // TODO: limit concurrency number
        spawn(async move {
            let res = some_computation(i).await;
            txc.send(res).await.unwrap();
        });
    }
    drop(tx);

    while let Some(res) = rx.recv().await {
        println!("~~~ {} {}", now(), res);
    }

    // broadcast
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    let t1 = spawn(async move {
        println!("~~~ rx1.recv: {}", rx1.recv().await.unwrap());
        println!("~~~ rx1.recv: {}", rx1.recv().await.unwrap());
    });

    let t2 = spawn(async move {
        println!("~~~ rx2.recv: {}", rx2.recv().await.unwrap());
        println!("~~~ rx2.recv: {}", rx2.recv().await.unwrap());
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
    join_all(vec![t1, t2]).await;

    // Arc<Mutex<T>>
    let data1 = Arc::new(Mutex::new(0));
    let data2 = data1.clone(); // Arc::clone(&data1);

    spawn(async move {
        let mut value = data2.lock().await;
        *value += 1;
        println!("~~~ data a: {}", value);
    });

    let mut value = data1.lock().await;
    *value += 1;
    println!("~~~ data b: {}", value);
}
