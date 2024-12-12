use num::traits::{CheckedAdd, CheckedSub, NumAssign, One, Zero};

use std::collections::BTreeMap;

/*
type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;
*/

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Copy + Zero + One + CheckedAdd + CheckedSub + NumAssign;
    type Nonce: Copy + Zero + One + Ord + CheckedAdd;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        // unimplemented!()
        // crash if overflow for purpose
        self.block_number = self.block_number.checked_add(&T::BlockNumber::one()).unwrap();
    }

    pub fn dec_block_number(&mut self) {
        self.block_number = self.block_number.checked_sub(&T::BlockNumber::one()).unwrap();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        // let nonce = self.nonce.get(who).unwrap_or(&0);
        // self.nonce.insert(who.clone(), nonce.checked_add(1).unwrap());

        let value = self.nonce.entry(who.clone()).or_insert(T::Nonce::zero());
        *value = value.checked_add(&T::Nonce::one()).unwrap();
    }

    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(&who.clone()).unwrap_or(&T::Nonce::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn map() {
        let mut map = BTreeMap::new();
        let alice = "alice".to_string();

        *map.entry(&alice).or_insert(0) += 1;
        assert_eq!(map.get(&alice), Some(&1));

        let mut value: u32 = 0;
        let p = &mut value;
        value = p.checked_add(1).unwrap_or(0);
        assert_eq!(value, 1);
    }

    #[test]
    fn init_system() {
        let pallet = Pallet::<TestConfig>::new();
        assert_eq!(pallet.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut pallet = Pallet::<TestConfig>::new();
        pallet.inc_block_number();
        assert_eq!(pallet.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let alice = "alice".to_string();
        let mut pallet = Pallet::<TestConfig>::new();

        pallet.inc_nonce(&alice);
        pallet.inc_nonce(&alice);
        assert_eq!(pallet.nonce.get(&alice), Some(&2));
    }
}
