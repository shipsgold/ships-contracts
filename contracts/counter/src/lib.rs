use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128, Base58PublicKey};
use near_sdk::{ext_contract, Balance, PublicKey, env, near_bindgen, setup_alloc, AccountId, PanicOnDefault, Promise, PromiseResult, StorageUsage};



use near_sdk::collections::LookupMap;

use std::collections::HashMap;
mod internal;
mod math;

setup_alloc!();

const ACCESS_KEY_ALLOWANCE: u128 = 100_000_000_000_000_000_000_000_000;
//const ACCESS_KEY_ALLOWANCE: u128 = 100_820_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    val: i8, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
    owner: ValidAccountId,
    // PublicKey -> AccountId.
    guests: LookupMap<PublicKey, AccountId>,
    accounts: LookupMap<AccountId, Balance>,
    account_storage_usage: StorageUsage,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId, count: Option<i8>) -> Self {
        assert!(
            !env::state_exists(),
            "Contract has already been initialized"
        );

        let val: i8;
        match count {
            None => val = 0,
            Some(count) => val = count,
        };
        let mut this = Self {
            val: val,
            owner: owner_id,
            guests: LookupMap::new(b"ga".to_vec()),
            accounts: LookupMap::new(b"aa".to_vec()),
            account_storage_usage: 0,
        };
        let initial_storage_usage = env::storage_usage();
        let tmp_account_id = unsafe { String::from_utf8_unchecked(vec![b'a'; 64]) };
        this.accounts.insert(&tmp_account_id, &0u128);
        this.account_storage_usage = env::storage_usage() - initial_storage_usage;
        this.accounts.remove(&tmp_account_id);
        this
    }

    pub fn add_guest(&mut self,  access_key: Base58PublicKey) {
        Promise::new(env::predecessor_account_id().clone())
        .add_access_key(access_key.into(), 
        ACCESS_KEY_ALLOWANCE, 
        env::current_account_id(),
        b"increment".to_vec());
    }

    pub fn new_owner(&mut self, owner_id: ValidAccountId) {
        self.owner = owner_id;
    }

    #[payable]
    pub fn increment(&mut self) -> i8{
        //self.assert_owner();
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
    use bs58;
    use near_sdk::test_utils::{accounts, get_logs, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk_sim::near_crypto::{InMemorySigner, KeyType, Signer};
    use near_sdk::{testing_env, VMContext, PublicKey};
    use std::convert::{TryFrom, TryInto};

    fn get_context(current_id: ValidAccountId, 
        predecessor_account_id: ValidAccountId, 
        signer_id: ValidAccountId,
        signer_pk: PublicKey) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(current_id)
            .signer_account_id(signer_id)
            .signer_account_pk(signer_pk)
            .attached_deposit(0)
            .account_balance(0)
            .account_locked_balance(0)
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn get_contract_id()-> ValidAccountId{
        accounts(1)
    }
    fn get_sponsor()->ValidAccountId {
        accounts(0)
    }
    fn get_sponsor_pk()->PublicKey {
        vec![1,2,3]
    }
    #[test]
    fn test_construction() {
        let mut context = get_context(get_sponsor(),
        get_contract_id(), 
        get_sponsor(),
        get_sponsor_pk());
        testing_env!(context.build());
        let contract = Contract::new(get_sponsor().into(), Some(9));
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.get_count(), 9);
    }
    #[test]
    fn test_partial() {
        let mut context = get_context(get_sponsor(),
        get_contract_id(), 
        get_sponsor(),
        get_sponsor_pk());
        testing_env!(context.build());
        let mut contract = Contract {
            owner: get_sponsor(),
            val: 0,
            guests: LookupMap::new(b"ga".to_vec()),
            accounts: LookupMap::new(b"aa".to_vec()),
            account_storage_usage: 0,
        };
        let fraction = math::SafeFraction {
            numerator: 1,
            denominator: 3,
        };
        println!("Value after increment: {}", fraction.multiply(1000));
    }

    #[test]
    fn test_basic() {
        let mut context = get_context(get_sponsor(),
        get_contract_id(), 
        get_sponsor(),
        get_sponsor_pk());

        let signer = InMemorySigner::from_seed("testuser", KeyType::ED25519, "testseed");
        let base_key = Base58PublicKey::try_from(signer.public_key().try_to_vec().unwrap()).unwrap();
        println!("{}",String::try_from(&base_key).unwrap());
        testing_env!(context.build());

        let mut contract = Contract {
            owner: accounts(1),
            val: 0,
            guests: LookupMap::new(b"ga".to_vec()),
            accounts: LookupMap::new(b"aa".to_vec()),
            account_storage_usage: 0,
        };
        contract.add_guest(base_key.clone());
        let spawned_user = ValidAccountId::try_from(String::from("sponsored_user")).unwrap();
        context = get_context(spawned_user.clone(),
        spawned_user, 
        ValidAccountId::try_from(String::from("sponsored_user")).unwrap(),
        vec![9,9,9,9,9]);
        testing_env!(context.build());
        contract.increment();
        println!("Value after increment: {}", contract.get_count());
        assert_eq!(1, contract.get_count());
    }

    //#[test]
    fn test_balance() {
       /* let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract {
            owner: accounts(1),
            val: 0,
            guests: LookupMap::new(b"ga".to_vec()),
            accounts: LookupMap::new(b"aa".to_vec()),
            account_storage_usage: 0,
        };
        let balance = env::account_balance();
        contract.increment();
        println!("Account balancd: {}", balance);
        assert_ne!(balance, 0);
        */
    }
}
