use chrono::{Local, SecondsFormat};
use tokio::{
    spawn,
    sync::mpsc,
    time::{sleep, Duration},
};

use std::error;

#[tokio::main]
async fn main() {
    println!("==> Signle Producer Single Consumer");
    run_spsc().await.unwrap();

    println!("==> Multipy Producer Single Consumer");
    run_mpsc().await;
}

async fn run_spsc() -> Result<(), Box<dyn error::Error>> {
    let (tx, mut rx) = mpsc::channel(1);

    let _handle = spawn(async move {
        for i in 0..10 {
            match tx.send(i).await {
                Ok(_) => println!("--> {} send: {i}", now()),
                Err(e) => {
                    eprintln!("--! {} send: {e:?}", now());
                    break;
                }
            }

            // sleep(Duration::new(1, 0)).await;
            sleep(Duration::from_secs(1)).await;
        }

        println!("--- {} finished sending", now());
    });

    let handle = spawn(async move {
        let mut count = 0;

        while let Some(_v) = rx.recv().await {
            count += 1;
            // println!("<-- {} got: {_v}", now());
        }

        println!("--- {} ended recv", now());
        count
    });

    let count = handle.await?;
    println!("--> Received {count} items");

    Ok(())
}

async fn run_mpsc() {
    let (tx, mut rx) = mpsc::channel(1);

    let tx_clone = tx.clone();
    spawn(async move {
        for i in 0..10 {
            match tx_clone.send(i).await {
                Err(e) => eprintln!("--! {} send: {e:?}", now()),
                Ok(_) => println!("--> {} send: {i}", now()),
            }
        }
    });

    let tx_clone = tx.clone();
    spawn(async move {
        for i in 10..20 {
            match tx_clone.send(i).await {
                Err(e) => eprintln!("--! {} send: {e:?}", now()),
                Ok(_) => println!("--> {} xxxx: {i}", now()),
            }
        }
    });

    drop(tx);

    let mut count = 0;

    while let Some(_v) = rx.recv().await {
        count += 1;
        // println!("<-- got: {_v}");
    }
    println!("--> Received {count} items");
}

fn now() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
