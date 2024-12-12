mod balances;
mod support;
mod system;

use support::Dispatch;

fn main() {
    // println!("Hello, world! WEB3DEV is cool!");

    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    /*
    runtime.balances.set_balance(&alice, 100);
    runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);
    runtime.system.inc_nonce(&alice);

    let _ = runtime.balances.transfer(&alice, &bob, 30).map_err(|e| println!("!!! 1. Error: {e}"));

    let _ =
        runtime.balances.transfer(&alice, &charlie, 20).map_err(|e| println!("!!! 2. Error: {e}"));
    */

    /*
    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::BalancesTransfer { to: bob.clone(), amount: 30 },
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::BalancesTransfer { to: charlie.clone(), amount: 20 },
            },
        ],
    };

    runtime.execute_block(block_1).expect("wrong block execution");
    */

    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: bob.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: charlie.clone(),
                    amount: 20,
                }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("wrong block execution");

    println!("{:#?}", runtime);
}

mod types {
    use crate::support;

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
}

pub enum RuntimeCall {
    // BalancesTransfer { to: types::AccountId, amount: types::Balance },
    Balances(balances::Call<Runtime>),
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

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();

        if self.system.block_number() != block.header.block_number {
            return Err("Block number mismatch");
        }

        let extrinsics = block.extrinsics.into_iter().enumerate();

        for (i, support::Extrinsic { caller, call }) in extrinsics {
            self.system.inc_nonce(&caller);
            // self.dispatch(caller, call)?;
            if let Err(e) = self.dispatch(caller, call) {
                eprintln!(
                    "Extrinsic Error: block_number={}, extrinsic_number={}, error={}",
                    block.header.block_number, i, e,
                );

                self.system.dec_block_number();
                return Err(e);
            }
        }

        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
        match call {
            //RuntimeCall::BalancesTransfer { to, amount } => {
            //    self.balances.transfer(&caller, &to, amount)?;
            //}
            RuntimeCall::Balances(call) => self.balances.dispatch(caller, call)?,
        }

        Ok(())
    }
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
