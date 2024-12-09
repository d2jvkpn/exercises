use num::traits::{CheckedAdd, CheckedSub, Zero};

use std::collections::BTreeMap;

/*
pub trait Config {
    type AccountId: Ord + Clone;
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}
*/

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: &T::AccountId,
        to: &T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);

        let caller_balance = caller_balance.checked_sub(&amount).ok_or("Insufficient balance")?;

        let to_balance =
            to_balance.checked_add(&amount).ok_or("Overflow when adding to balance")?;

        self.set_balance(&caller, caller_balance);
        self.set_balance(&to, to_balance);

        Ok(())
    }
}

pub enum Call<T: Config> {
    Tranfer { to: T::AccountId, amount: T::Balance },
    // RemoveMe{core::marker::PhantomData{}},
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> crate::support::DispatchResult {
        match call {
            Call::Tranfer { to, amount } => self.transfer(&caller, &to, amount),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut pallet = Pallet::<TestConfig>::new();

        assert_eq!(pallet.balance(&"alice".to_string()), 0);
        pallet.set_balance(&"alice".to_string(), 100);
        assert_eq!(pallet.balance(&"alice".to_string()), 100);
    }

    #[test]
    fn transfer_balance() {
        let mut pallet = Pallet::<TestConfig>::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        pallet.set_balance(&alice, 100);

        let _ = pallet.transfer(&alice, &bob, 90);
        assert_eq!(pallet.balance(&alice), 10);
        assert_eq!(pallet.balance(&bob), 90);
    }

    #[test]
    fn transfer_balance_insufficent() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut pallet = Pallet::<TestConfig>::new();
        pallet.set_balance(&alice, 100);

        let result = pallet.transfer(&alice, &bob, 200);

        assert_eq!(result, Err("Insufficient balance"));
        assert_eq!(pallet.balance(&alice), 100);
        assert_eq!(pallet.balance(&bob), 0);
    }

    #[test]
    fn transfer_balance_overflow() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        let mut pallet = Pallet::<TestConfig>::new();

        pallet.set_balance(&alice, 100);
        pallet.set_balance(&bob, u128::MAX);

        let result = pallet.transfer(&alice, &bob, 1);

        assert_eq!(result, Err("Overflow when adding to balance"));
        assert_eq!(pallet.balance(&alice), 100);
        assert_eq!(pallet.balance(&bob), u128::MAX);
    }
}
