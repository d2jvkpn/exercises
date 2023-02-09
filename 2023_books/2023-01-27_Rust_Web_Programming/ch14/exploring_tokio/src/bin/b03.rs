use std::fmt;
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    oneshot,
};

#[derive(Debug)]
pub enum Order {
    BUY,
    SELL,
}

#[derive(Debug)]
pub struct Message {
    pub order: Order,
    pub ticker: String,
    pub amount: f32,
    pub respond_to: oneshot::Sender<bool>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, {}, {}", self.order, self.ticker, self.amount)
    }
}

pub struct OrderBookActor {
    pub receiver: Receiver<Message>,
    pub total_invested: f32,
    pub investment_cap: f32,
}

impl OrderBookActor {
    fn new(receiver: Receiver<Message>, investment_cap: f32) -> Self {
        Self { receiver, total_invested: 0.0, investment_cap }
    }

    fn handle(&mut self, msg: Message) {
        if msg.amount + self.total_invested <= self.investment_cap {
            self.total_invested += msg.amount;
            println!("--> processing purchase ({}), total invested: {}", msg, self.total_invested,);
            let _ = msg.respond_to.send(true);
        } else {
            println!("!!! rejecting purchase ({}), total invested: {}", msg, self.total_invested,);
            let _ = msg.respond_to.send(false);
        }
    }

    async fn run(mut self) {
        println!("==> Actor is running");

        while let Some(msg) = self.receiver.recv().await {
            self.handle(msg);
        }
    }
}

pub struct BuyOrder {
    pub ticker: String,
    pub amount: f32,
    pub order: Order,
    pub sender: Sender<Message>,
}

impl BuyOrder {
    fn new(amount: f32, ticker: String, sender: Sender<Message>) -> Self {
        Self { ticker, amount, sender, order: Order::BUY }
    }

    async fn send(self) {
        let (send, recv) = oneshot::channel();
        let message = Message {
            order: self.order,
            amount: self.amount,
            ticker: self.ticker,
            respond_to: send,
        };

        let _ = self.sender.send(message).await;

        match recv.await {
            Ok(v) => println!("~~~ order result: {}", if v { "ACCEPETD" } else { "REJECTED" }),
            Err(e) => eprintln!("!!! order recv error: {e:?}"),
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = channel::<Message>(1);

    let tx1 = tx.clone();
    tokio::spawn(async move {
        for _ in 0..5 {
            let buyer = BuyOrder::new(5.5, "BYND".into(), tx1.clone());
            buyer.send().await;
        }
    });

    let tx2 = tx.clone();
    tokio::spawn(async move {
        for _ in 0..5 {
            let buyer = BuyOrder::new(5.5, "PLR".into(), tx2.clone());
            buyer.send().await;
        }
    });

    drop(tx);

    let actor = OrderBookActor::new(rx, 20.0);
    actor.run().await;
}
