mod sorting;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/

/// A savings account
pub struct SavingsAccount {
    balance: f32,
}

impl SavingsAccount {
    /// Creates a a `SavingsAccount` a balance of 0
    ///
    /// # Example
    /// ```
    /// use utils::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(), 0.0);
    /// ```
    pub fn new() -> Self {
        Self { balance: 0.0 }
    }

    pub fn get_balance(&self) -> f32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: f32) {
        if amount < 0.0 {
            panic!("Can't deposit a negative amount!");
        }
        self.balance += amount;
    }

    pub fn transfer(&mut self, acc_number: i32, amount: f32) -> Result<(), &'static str> {
        if acc_number <= 0 {
            return Err("invalid acc_number");
        }

        if amount > self.balance {
            return Err("no enough deposit");
        }

        self.balance -= amount;
        dbg!("Transferred ${amount} to ${acc_number}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingsAccount::new();

        assert_eq!(account.get_balance(), 0.0);

        account.deposit(100.0);
        assert_eq!(account.get_balance(), 100.0, "Balance should be 100!");

        assert_ne!(account.get_balance(), 0.0);
        assert!(account.get_balance() > 0.0);
    }

    #[test]
    fn should_transfer_money() -> Result<(), &'static str> {
        let mut account = SavingsAccount::new();
        account.deposit(100.0);
        account.transfer(123456, 100.0)?;
        Ok(())
    }

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let mut account = SavingsAccount::new();
        account.deposit(-1.0);
    }
}
