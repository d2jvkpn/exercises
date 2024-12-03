use num::traits::{CheckedAdd, CheckedSub, Zero};

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
    AccountId: Ord + Clone,
    Balance: Zero + CheckedSub + CheckedAdd + Copy,
{
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: &AccountId,
        to: &AccountId,
        amount: Balance,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_balances() {
        let mut pallet = Pallet::<String, u128>::new();

        assert_eq!(pallet.balance(&"alice".to_string()), 0);
        pallet.set_balance(&"alice".to_string(), 100);
        assert_eq!(pallet.balance(&"alice".to_string()), 100);
    }

    #[test]
    fn transfer_balance() {
        let mut pallet = Pallet::<String, u128>::new();
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

        let mut pallet = Pallet::<String, u128>::new();
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
        let mut pallet = Pallet::<String, u128>::new();

        pallet.set_balance(&alice, 100);
        pallet.set_balance(&bob, u128::MAX);

        let result = pallet.transfer(&alice, &bob, 1);

        assert_eq!(result, Err("Overflow when adding to balance"));
        assert_eq!(pallet.balance(&alice), 100);
        assert_eq!(pallet.balance(&bob), u128::MAX);
    }
}
