use std::{collections::HashMap, fmt};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    oneshot,
};

#[derive(Debug, Clone)]
pub enum Order {
    Buy(String, f32),
    Get,
}

#[derive(Debug)]
pub struct TrackerMessage {
    pub command: Order,
    pub respond_to: oneshot::Sender<String>,
}

impl fmt::Display for TrackerMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.command {
            Order::Get => write!(f, "GET"),
            Order::Buy(ticker, amount) => write!(f, "BUY/{ticker}/{amount:.2}"),
        }
    }
}

pub struct GetTrackerActor {
    pub sender: Sender<TrackerMessage>,
}

impl GetTrackerActor {
    pub async fn send(self) -> String {
        // println!("~~~ GET function firing");
        let (send, recv) = oneshot::channel();
        let msg = TrackerMessage { command: Order::Get, respond_to: send };
        let _ = self.sender.send(msg).await;

        recv.await.unwrap_or_else(|e| format!("!!!{e:?}"))
    }
}

pub struct TrackerActor {
    pub receiver: Receiver<TrackerMessage>,
    pub db: HashMap<String, f32>,
}

impl TrackerActor {
    pub fn new(receiver: Receiver<TrackerMessage>) -> Self {
        Self { receiver, db: HashMap::new() }
    }

    pub fn send_state(&self, respond_to: oneshot::Sender<String>) {
        let mut buffer = Vec::new();
        for (k, v) in &self.db {
            buffer.push(format!("{k}:{v:.2}"));
        }
        let state = buffer.join(";");
        let _ = respond_to.send(state);
    }

    fn handle(&mut self, msg: TrackerMessage) {
        match msg.command {
            Order::Get => self.send_state(msg.respond_to),
            Order::Buy(ticker, amount) => *self.db.entry(ticker).or_insert(0_f32) += amount,
        };
    }

    pub async fn run(mut self) {
        println!("==> TrackerActor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle(msg);
        }
    }
}
