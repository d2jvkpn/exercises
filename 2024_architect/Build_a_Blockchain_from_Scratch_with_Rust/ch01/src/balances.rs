use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: &String,
        to: &String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);

        let caller_balance = caller_balance.checked_sub(amount).ok_or("Insufficient balance")?;

        let to_balance = to_balance.checked_add(amount).ok_or("Overflow when adding to balance")?;

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
        let mut pallet = Pallet::new();
        let alice = "alice".to_string();

        assert_eq!(pallet.balance(&alice), 0);
        pallet.set_balance(&alice, 100);
        assert_eq!(pallet.balance(&alice), 100);
    }

    #[test]
    fn transfer_balance() {
        let mut pallet = Pallet::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        pallet.set_balance(&alice, 100);

        pallet.transfer(&alice, &bob, 90).unwrap();
        assert_eq!(pallet.balance(&alice), 10);
        assert_eq!(pallet.balance(&bob), 90);
    }

    #[test]
    fn transfer_balance_insufficent() {
        let mut pallet = Pallet::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        pallet.set_balance(&alice, 100);

        let result = pallet.transfer(&alice, &bob, 200);

        assert_eq!(result, Err("Insufficient balance"));
        assert_eq!(pallet.balance(&alice), 100);
        assert_eq!(pallet.balance(&bob), 0);
    }

    #[test]
    fn transfer_balance_overflow() {
        let mut pallet = Pallet::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        pallet.set_balance(&alice, 100);
        pallet.set_balance(&bob, u128::MAX);

        let result = pallet.transfer(&alice, &bob, 1);

        assert_eq!(result, Err("Overflow when adding to balance"));
        assert_eq!(pallet.balance(&alice), 100);
        assert_eq!(pallet.balance(&bob), u128::MAX);
    }
}
