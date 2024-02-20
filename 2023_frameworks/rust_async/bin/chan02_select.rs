use chrono::{Local, SecondsFormat};
use tokio::{
    select,
    spawn,
    sync::{broadcast, mpsc}, // oneshot
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(1);
    let (tx2, mut rx2) = broadcast::channel(1);

    let _handle = spawn(async move {
        for i in 0..10 {
            select! {
                v1 = rx2.recv() => {
                    match v1 {
                    Ok(v) => println!("--> {} got broadcast: {v}", now()),
                    Err(e) =>  eprintln!("!!! {} got broadcast: {e}", now()),
                    }

                    return;
                }

                v2 = tx1.send(i) => {
                    if let Err(e) = v2 {
                        eprintln!("!!! {} send: {i}, {e:?}", now());
                        return;
                    }
                }
            };

            println!("--> {} send: {i}", now());
            // sleep(Duration::new(1, 0)).await;
            sleep(Duration::from_secs(1)).await;
        }
    });

    let handle = spawn(async move {
        let mut n = 0;

        while let Some(_v) = rx1.recv().await {
            n += 1;
            // println!("<-- {} got: {v}", now());
        }

        n
    });

    sleep(Duration::new(5, 0)).await;
    tx2.send(false).unwrap(); // make _handle exit earlier
    println!("==> Received {} items", handle.await.unwrap());
}

fn now() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
