use chrono::{Local, SecondsFormat};
use futures::{future, select};

fn main() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let (mut total, mut count) = (0, 0);

    loop {
        count += 1;

        select! {
            a = a_fut => {
                println!("~~~ {} a: {a}", now_string());
                total += a;
                // break; //
            }

            b = b_fut => {
                println!("~~~ {} b: {b}", now_string());
                total += b;
            }

            complete => {
                println!("~~~ {} done.", now_string());
                break;
            }

            default => panic!(), // never runs (futures run first, then complete)
        };
    }

    println!("~~~ loop count: {count}");
    assert_eq!(total, 10);
}

fn now_string() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
