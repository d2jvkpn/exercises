// use once_cell::sync::OnceCell;

use std::{sync::Once, thread};

static mut CONFIG: Option<String> = None;
static INIT: Once = Once::new();

fn init_config() {
    INIT.call_once(|| {
        println!("==> Initializing global configuration.");

        unsafe {
            CONFIG = Some("Configuration.yaml".to_string());
        }
    });
}

fn get_config() -> &'static str {
    unsafe { CONFIG.as_ref().unwrap() }
}

fn main() {
    init_config();
    init_config();

    println!("{}", get_config());

    let h1 = thread::spawn(|| {
        println!("~~~ thread1: {}", get_config());
    });

    let h2 = thread::spawn(|| {
        println!("~~~ thread2: {}", get_config());
    });

    h1.join().unwrap();
    h2.join().unwrap();
}
