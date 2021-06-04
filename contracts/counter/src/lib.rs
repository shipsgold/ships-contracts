use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, PanicOnDefault};

use std::collections::HashMap;
mod internal;
mod math;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    val: i8, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
    owner: ValidAccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId, count: Option<i8>) -> Self {
       /* assert!(
            !env::state_exists(),
            "Contract has already been initialized"
        );*/
        let mut val: i8;
        match count {
            None => val = 0,
            Some(count) => val = count,
        };
        Self {
            val: val,
            owner: owner_id,
        }
    }

    pub fn new_owner(&mut self, owner_id: ValidAccountId) {
        self.owner = owner_id;
    }

    #[payable]
    pub fn increment(&mut self) -> i8{
        self.assert_owner();
        self.val += 1;
        return self.val;
    }
    pub fn get_count(&self) -> i8 {
        self.val
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, get_logs, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use std::convert::TryInto;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }
    #[test]
    fn test_construction() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Counter::new(accounts(1).into(), Some(9));
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.get_count(), 9);
    }
    #[test]
    fn test_partial() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Counter {
            owner: accounts(1),
            val: 0,
        };
        let fraction = math::SafeFraction {
            numerator: 1,
            denominator: 3,
        };
        println!("Value after increment: {}", fraction.multiply(1000));
    }

    #[test]
    fn test_basic() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Counter {
            owner: accounts(1),
            val: 0,
        };
        contract.increment();
        println!("Value after increment: {}", contract.get_count());
        assert_eq!(1, contract.get_count());
    }

    #[test]
    fn test_balance() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Counter {
            owner: accounts(1),
            val: 0,
        };
        let balance = env::account_balance();
        contract.increment();
        println!("Account balancd: {}", balance);
        assert_ne!(balance, 0);
    }
}
