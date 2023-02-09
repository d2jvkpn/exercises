use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Order {
    BUY,
    SELL,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub order: Order,
    pub ticker: String,
    pub amount: f32,
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Message>(1);

    let orders = [
        Message { order: Order::BUY, amount: 5.5, ticker: "BYND".to_owned() },
        Message { order: Order::BUY, amount: 5.5, ticker: "NET".to_owned() },
        Message { order: Order::BUY, amount: 5.5, ticker: "PLTR".to_owned() },
    ];

    tokio::spawn(async move {
        for v in orders {
            if let Err(e) = tx.send(v.clone()).await {
                eprintln!("send error: {e:?}");
            }
            println!("--> send: {v:?}");
        }
    });

    let mut n = 0;
    while let Some(v) = rx.recv().await {
        n += 1;
        println!("<-- got: {v:?}");
    }

    println!("==> Received {n} items");
}
