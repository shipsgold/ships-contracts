use crate::*;
#[near_bindgen]
impl Contract {

    pub fn add_guest(&mut self,  access_key: PublicKey) {
        self.guests.insert(&access_key.clone().into());
        Promise::new(env::predecessor_account_id().clone())
        .add_access_key(access_key.into(), 
        ACCESS_KEY_ALLOWANCE, 
        env::current_account_id(),
        "increment".to_string());

    }

    pub fn new_owner(&mut self, owner_id: AccountId) {
        self.owner = owner_id;
    }

    pub fn is_guest(&self, access_key: PublicKey) -> bool{
        self.guests.contains(&access_key.into())
    }

}

