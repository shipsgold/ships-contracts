use crate::*;
use near_sdk::{require};
impl Contract {
  pub(crate) fn assert_owner(&self) {
    require!(
      env::predecessor_account_id().to_string() == self.owner.to_string(),
      "Can only be called by the owner"
    );
  }
}
