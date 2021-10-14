use crate::*;

#[near_bindgen]
impl Contract {

    pub fn add_guest(&mut self,  access_key: PublicKey) {

    }
    pub fn add_temp_user(&mut self, user_id: String, access_key: PublicKey) {
        if let Some(_user_key) = self.temp_users.insert(&user_id, &access_key.clone().into()) {
                env::panic_str("Username already exists");
        }
        // should be comma seperated list like "methoda,methodb".to_string()
        let method_names = "increment".to_string();
        Promise::new(env::predecessor_account_id().clone())
            .add_access_key(access_key.into(),
                            ACCESS_KEY_ALLOWANCE,
                            env::current_account_id(),
                            method_names);
    }

    pub fn reset_temp_user(&mut self, user_id: String, access_key: PublicKey) {
        require!(env::predecessor_account_id() == self.owner ||
        env::predecessor_account_id() == self.verifier);
        // TODO leaks memory needs clean up of all the places the user_id has been
        self.temp_users.insert(&user_id, &access_key);
    }

    pub fn get_temp_user(&self, user_id: String) -> Option<PublicKey>{
        self.temp_users.get(&user_id.into())
    }

}

