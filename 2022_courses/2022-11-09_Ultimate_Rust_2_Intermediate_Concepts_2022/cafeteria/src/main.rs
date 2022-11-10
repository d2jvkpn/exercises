use chrono::{Local, SecondsFormat};
use crossbeam::channel::{self, Receiver, Sender};

use std::thread;
use std::time::{self, Duration};

#[derive(Debug)]
#[non_exhaustive]
enum Lunch {
    Soup,
    Salad,
    Sandwich,
    HotDog,
}

fn now_string() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn cafeteria_worker(name: &str, orders: Receiver<&str>, lunches: Sender<Lunch>) {
    for order in orders {
        println!(
            "{} <== {} receives an order for {:?}, {}s",
            now_string(),
            name,
            order,
            order.len()
        );

        let order = order.to_lowercase();

        let lunch = match &order {
            x if x.contains("soup") => Lunch::Soup,
            x if x.contains("salad") => Lunch::Salad,
            x if x.contains("sandwich") => Lunch::Sandwich,
            _ => Lunch::HotDog,
        };

        //        for _ in 0..order.len() {
        //            thread::sleep(Duration::from_secs_f32(0.1));
        //        }
        thread::sleep(Duration::from_secs(order.len() as u64));

        println!("{} ==> {} sends a {:?}", now_string(), name, lunch);
        if lunches.send(lunch).is_err() {
            break;
        }
    }
}

fn main() {
    let now = time::Instant::now();

    let (orders_tx, orders_rx) = channel::unbounded();
    let (lunches_tx, lunches_rx) = channel::unbounded();
    //    let (orders_tx, orders_rx) = channel::bounded(2);
    //    let (lunches_tx, lunches_rx) = channel::bounded(2);

    let (orders, lunches) = (orders_rx.clone(), lunches_tx.clone());
    let alice = thread::spawn(|| cafeteria_worker("alice", orders, lunches));

    let (orders, lunches) = (orders_rx.clone(), lunches_tx.clone());
    let zack = thread::spawn(|| cafeteria_worker("zack", orders, lunches));

    let food = vec!["polish dog", "caesar salad", "oion soup", "reuben sandwich"];

    //    for order in &food {
    //        println!("{} >>> ORDER: {:?}", now_string(), order);
    //        let _ = orders_tx.send(order);
    //    }
    food.iter().for_each(|v| {
        println!("{} >>> ORDER: {:?}", now_string(), v);
        let _ = orders_tx.send(v);
    });
    drop(orders_tx);

    //    drop(lunches_tx);
    //    let mut count = 0;
    //    for lunch in lunches_rx {
    //        println!("{} === Order up! -> {:?}", now_string(), lunch);
    //        count += 1;
    //    }
    //    assert_eq!(food.len(), count);

    //    lunches_rx.iter().for_each(|v| {
    //        println!("{} === Order up! -> {:?}", now_string(), v);
    //    });

    for _ in 0..food.len() {
        let lunch = lunches_rx.recv();
        println!("{} === Order up! -> {:?}", now_string(), lunch);
    }

    println!("{} ~~~ Join threads", now_string());
    let _ = alice.join();
    let _ = zack.join();

    println!("Elapsed: {:?}", Duration::from_millis(now.elapsed().as_millis().try_into().unwrap()));
}
