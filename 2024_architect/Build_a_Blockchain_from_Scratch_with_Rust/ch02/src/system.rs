use std::collections::BTreeMap;

pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { block_number: 0, nonce: BTreeMap::new() }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        // unimplemented!()
        // crash if overflow for purpose
        self.block_number = self.block_number.checked_add(1).unwrap();
    }

    pub fn inc_nonce(&mut self, who: &String) {
        // let nonce = self.nonce.get(who).unwrap_or(&0);
        // self.nonce.insert(who.clone(), nonce.checked_add(1).unwrap());

        let mut value = self.nonce.entry(who.to_string()).or_insert(0);
        *value = value.checked_add(1).unwrap();
    }

    pub fn get_nonce(&self, who: &String) -> u32 {
        *self.nonce.get(&who.clone()).unwrap_or(&0)
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
        let pallet = Pallet::new();
        assert_eq!(pallet.block_number(), 0);
    }

    #[test]
    fn inc_block_number() {
        let mut pallet = Pallet::new();
        pallet.inc_block_number();
        assert_eq!(pallet.block_number(), 1);
    }

    #[test]
    fn inc_nonce() {
        let alice = "alice".to_string();
        let mut pallet = Pallet::new();

        pallet.inc_nonce(&alice);

        assert_eq!(pallet.nonce.get(&alice), Some(&1));
    }
}
