use std::fmt;
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    oneshot,
};

#[derive(Debug)]
pub enum Kind {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct Message {
    pub kind: Kind,
    pub ticker: String,
    pub amount: f32,
    pub respond_to: oneshot::Sender<bool>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}/{}/{}", self.kind, self.ticker, self.amount)
    }
}

pub struct BookActor {
    pub receiver: Receiver<Message>,
    pub total_count: u32,
    pub total_invested: f32,
    pub investment_cap: f32,
}

impl fmt::Display for BookActor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{:.2}/{:.2}", self.total_count, self.total_invested, self.investment_cap)
    }
}

impl BookActor {
    pub fn new(receiver: Receiver<Message>, investment_cap: f32) -> Self {
        Self { receiver, total_count: 0, total_invested: 0.0, investment_cap }
    }

    fn handle(&mut self, msg: Message) {
        if msg.amount + self.total_invested <= self.investment_cap {
            self.total_invested += msg.amount;
            self.total_count += 1;
            println!("--> processing purchase {msg}, invested: {self}");
            let _ = msg.respond_to.send(true);
        } else {
            println!("!!! rejecting purchase {msg}, invested: {self}");
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
    pub kind: Kind,
    pub sender: Sender<Message>,
}

impl BuyOrder {
    pub fn new(amount: f32, ticker: String, sender: Sender<Message>) -> Self {
        Self { ticker, amount, sender, kind: Kind::Buy }
    }

    pub async fn send(self) {
        let (send, recv) = oneshot::channel();
        let message =
            Message { kind: self.kind, amount: self.amount, ticker: self.ticker, respond_to: send };

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

    let txc = tx.clone();
    tokio::spawn(async move {
        for _ in 0..5 {
            let buyer = BuyOrder::new(5.5, "BYND".into(), txc.clone());
            buyer.send().await;
        }
    });

    for _ in 0..5 {
        let txc = tx.clone();
        tokio::spawn(async move {
            let buyer = BuyOrder::new(3.5, "PLR".into(), txc.clone());
            buyer.send().await;
        });
    }

    drop(tx);

    let actor = BookActor::new(rx, 20.0);
    actor.run().await;
}
