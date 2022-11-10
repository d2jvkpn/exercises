use futures::executor::block_on;
use futures::future::{join, ready, FutureExt}; // FutureExt // for `.fuse()`
use futures::{pin_mut, select};

async fn count() {
    let mut a_fut = ready(4);
    let mut b_fut = ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => {
                println!("~~~ add a, {}, {}", total, a);
                total += a;
            },
            b = b_fut => {
                println!("~~~ add b, {}, {}", total, b);
                total += b
            },
            complete => {
                println!("~~~ complete1");
                break;
            },
            // default => unreachable!(),
        };
    }
    dbg!(&total);

    loop {
        select! {
            a = a_fut => {
                // won't run
                println!("==> add a, {}, {}", total, a);
                total += a;
            },
            b = b_fut => {
                // won't run
                println!("==> add b, {}, {}", total, b);
                total += b
            },
            complete => {
                println!("==> complete2");
                break;
            },
            // default => unreachable!(),
        };
    }
    dbg!(&total);

    assert_eq!(total, 10);
}

async fn task_one() {
    println!("... task_one");
}

async fn task_two() {
    println!("... task_two");
}

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    select! {
        () = t1 => println!("~~~ task one completed first"),
        () = t2 => println!("~~~ task two completed first"),
    }

    select! {
        () = t1 => println!("==> task one completed first"),
        () = t2 => println!("==> task two completed first"),
    }
}

fn main() {
    // block_on(count());
    // block_on(race_tasks());

    block_on(join(count(), race_tasks()));
    // block_on(join(race_tasks(), count()));
}
