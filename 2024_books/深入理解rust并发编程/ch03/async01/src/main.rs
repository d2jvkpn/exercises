// use tokio::{sync, time, net, fs, runtime, task, stream};

use tokio::{spawn, task::spawn_blocking};

#[tokio::main]
async fn main() {
    let fut01 = async {
        println!("==> Hello from fut01!");

        spawn(async {
            println!("Hello from a tokio task!");
            println!("in spawn");
        })
        .await
        .unwrap();
    };

    fut01.await;

    spawn_blocking(|| println!("==> in spawn_blocking!"));
}
