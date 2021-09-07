    use near_sdk::{PromiseOrValue, BorshStorageKey, ext_contract, Balance, PublicKey, env, near_bindgen, AccountId, PanicOnDefault, Promise, PromiseResult, StorageUsage};
    use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
    use near_sdk::json_types::{U128, U64};
    use near_sdk::serde::{Serialize, Deserialize};
    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use std::convert::TryFrom;

    pub fn get_context(current_id: AccountId,
                   predecessor_account_id: AccountId,
                   signer_id: AccountId,
                   signer_pk: PublicKey) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(current_id)
            .signer_account_id(signer_id)
            .signer_account_pk(signer_pk)
            .attached_deposit(134000000000000000000000)
            .account_balance(134000000000000000000000)
            .account_locked_balance(0)
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    pub fn get_contract_id() -> AccountId {
        accounts(1)
    }

    pub fn get_sponsor() -> AccountId {
        accounts(0)
    }

    pub fn get_sponsor_pk() -> PublicKey {
        "ed25519:6E8sCci9badyRkXb3JoRpBj5p8C6Tw41ELDZoiihKEtp".parse()
                     .unwrap()
    }