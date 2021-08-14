use crate::*;
#[near_bindgen]
impl Contract {

    pub fn add_guest(&mut self,  access_key: Base58PublicKey) {
        self.guests.insert(&access_key.clone().into());
        Promise::new(env::predecessor_account_id().clone())
        .add_access_key(access_key.into(), 
        ACCESS_KEY_ALLOWANCE, 
        env::current_account_id(),
        b"increment".to_vec());

    }

    pub fn new_owner(&mut self, owner_id: ValidAccountId) {
        self.owner = owner_id;
    }

    pub fn is_guest(&self, access_key: Base58PublicKey) -> bool{
        self.guests.contains(&access_key.into())
    }

}

