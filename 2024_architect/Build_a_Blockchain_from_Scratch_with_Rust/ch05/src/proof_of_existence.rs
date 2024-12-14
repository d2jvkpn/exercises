use crate::support::{Dispatch, DispatchResult};

use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self { claims: BTreeMap::new() }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.get_claim(&claim).is_some() {
            return Err("Claim already exists");
        }

        self.claims.insert(claim, caller);
        Ok(())
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let claim_owner = self.get_claim(&claim).ok_or("Claim does not exists")?;

        if claim_owner != &caller {
            return Err("Caller is not the owner of the claim");
        }

        self.claims.remove(&claim);

        Ok(())
    }
}

pub enum Call<T: Config> {
    CreateClaim { claim: T::Content },
    RevokeClaim { claim: T::Content },
    // RemoveMe(core::marker::PhantomData<T>)
}

impl<T: Config> Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim { claim } => self.create_claim(caller, claim),
            Call::RevokeClaim { claim } => self.revoke_claim(caller, claim),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl Config for TestConfig {
        type Content = &'static str;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe = Pallet::<TestConfig>::new();

        let alice = "alice";
        let claim = "my_document";
        poe.create_claim(&alice, claim).unwrap();

        assert_eq!(poe.get_claim(&claim), Some(&"alice"));

        //
        assert_eq!(poe.revoke_claim("bob", claim), Err("Caller is not the owner of the claim"));

        assert_eq!(poe.create_claim("bob", claim), Err("Claim already exists"));

        //
        assert_eq!(poe.revoke_claim(&alice, "non existant"), Err("Claim does not exists"));

        //
        poe.revoke_claim(&alice, claim).unwrap();
        assert!(poe.get_claim(&claim).is_none());
    }
}
