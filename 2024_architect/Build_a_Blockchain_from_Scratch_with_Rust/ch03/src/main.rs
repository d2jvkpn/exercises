mod balances;
mod system;

fn main() {
    // println!("Hello, world! WEB3DEV is cool!");
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);
    runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);
    runtime.system.inc_nonce(&alice);

    let _ = runtime.balances.transfer(&alice, &bob, 30).map_err(|e| println!("!!! 1. Error: {e}"));

    let _ =
        runtime.balances.transfer(&alice, &charlie, 20).map_err(|e| println!("!!! 2. Error: {e}"));

    println!("{:#?}", runtime);
}

#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<Runtime>,
    system: system::Pallet<Runtime>,
}

impl Runtime {
    pub fn new() -> Self {
        Self { balances: balances::Pallet::new(), system: system::Pallet::new() }
    }
}

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    // type AccountId = types::AccountId;
    type Balance = types::Balance;
}
