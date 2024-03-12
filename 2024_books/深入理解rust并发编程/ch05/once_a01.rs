use std::sync::Once;

static mut CONFIG: Option<String> = None;
static INIT: Once = Once::new();

fn init_config() {
    println!("==> Initializing global configuration.");

    unsafe {
        CONFIG = Some("The Configuration.".to_string());
    }
}

fn get_config() -> &'static str {
    INIT.call_once(|| init_config());

    unsafe { CONFIG.as_ref().unwrap() }
}

fn main() {
    println!("{}", get_config());
    println!("{}", get_config());
}
