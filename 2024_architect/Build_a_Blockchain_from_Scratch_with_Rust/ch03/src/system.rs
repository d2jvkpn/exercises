use num::traits::{CheckedAdd, CheckedSub, NumAssign, One, Zero};

use std::collections::BTreeMap;

/*
type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;
*/

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord + Clone,
    BlockNumber: Copy + Zero + One + CheckedAdd + CheckedSub + NumAssign,
    Nonce: Copy + Ord + Zero + One + Clone + CheckedAdd,
{
    pub fn new() -> Self {
        Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        // unimplemented!()
        // crash if overflow for purpose
        self.block_number = self.block_number.checked_add(&BlockNumber::one()).unwrap();
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        // let nonce = self.nonce.get(who).unwrap_or(&0);
        // self.nonce.insert(who.clone(), nonce.checked_add(1).unwrap());

        let value = self.nonce.entry(who.clone()).or_insert(Nonce::zero());
        *value = value.checked_add(&Nonce::one()).unwrap();
    }

    pub fn get_nonce(&self, who: &AccountId) -> Nonce {
        *self.nonce.get(&who.clone()).unwrap_or(&Nonce::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let pallet = Pallet::<String, u32, u32>::new();
        assert_eq!(pallet.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut pallet = Pallet::<String, u32, u32>::new();
        pallet.inc_block_number();
        assert_eq!(pallet.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let alice = "alice".to_string();
        let mut pallet = Pallet::<String, u32, u32>::new();

        pallet.inc_nonce(&alice);
        pallet.inc_nonce(&alice);
        assert_eq!(pallet.nonce.get(&alice), Some(&2));
    }
}
