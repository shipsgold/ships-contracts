use near_sdk::env;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use crate::*;


impl Contract {
    /*
    /// Placeholder for registering identity
    /// Registers external verifier for the project so it can verify identity of user offchain and report
    /// back
    pub fn register_ext_identity_verifier(&mut self, project_id: ProjectId, verifier: AccountId) {

    }*/
    /// Here user id can be anything could be public key in case of a temp user or account name
    /// Function is used to create an entry for the token for the username specified, it is used
    /// to allow a user to transfer funds immediately to the claimable space, if the user
    /// does have an account already this function is a gate way that will clear the storage
    /// space and transfer the tokens once the account is claimed to an existing account
    pub fn register_ext_user(&mut self, project_id: ProjectId, user: String, user_id:String){

    }

    /// Function is used to send funds to the specified user it sends tokens to the temporary user
    /// assuming that the temporary user will link the account this allows the developer to simply
    /// send a link that can point a user to page to call the link_ext_user functionality, which will
    /// complete the final transfer, by removing the existing entry and writing a new one reclaiming
    /// the storage spent on the registering of the user as funds get transferred to the verifying account
    /// when the account the verifier triggers a transfer. The verifier is always a source of potential
    /// foul play as the verifier could "fraud". Ideally verification only happens for small amounts,
    /// and users create either temporary accounts taht are then used in smart contract lookup instead of
    /// transfer so ths transfer functionality goes directly to the registered or linked account.
    pub fn transfer_ext_user(&mut self, project_id: ProjectId, release_id: ReleaseId, user: String, amount: U128){

    }

    /// The linked account functionality can only be called by the project verifier or the user that
    /// already has a linked identity. stored. The logic here is that both entities can relay
    /// the linkage of the external user identity and the project contract identity (aka near
    /// account or temporary pub/priv key pair)
    ///
    pub fn link_ext_user(&mut self, project_id:ProjectId, user:String, user_id: String) {


    }

    /// The linked account functionality can only be called by the project verifier or the user that
    /// already has a linked identity. stored. The logic here is that both entities can relay
    /// the linkage. The transfer part will transfer any existing linkage value to the new user
    /// and destroy the account freeing up the space, sending the allocation back to the creator
    /// of the temp account.
    pub fn link_ext_user_and_transfer(&mut self, project_id:ProjectId, user:String, user_id: String) {


    }

}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::*;
    #[test]
    pub fn basic_test() {
        let mut context = get_context(get_sponsor(),
                                      get_contract_id(),
                                      get_sponsor(),
                                      get_sponsor_pk());
        testing_env!(context.build());
        /*
        let contract = Contract::new(get_sponsor().into());
        contract.register_external_identity_verifier(project_id, verifier_id);
        contract.register_external_user(foodbar, pk:foodbar);
        contract.unregister_external_user(foodbar, project_id); // project owner can do this
        internal.get_account_for_external_user(foodbar);
        transfer_to_external_user(foodbar, release_id, project_id, 100);
        // developer creates a link by calling
        register_external_user(foodbar, project_id, place_holder_id); // creates an empty entry if it doesn't exist
        transfer_external_user(foodbar, 100); // if user doesn't exist it transfers tokens to
        a unique place holder account id; until user can verify
        //in external_user_registry
        // User clicks on link goes to website that makes a request to the api to verify identity
        // claim with new account
        // claim with existing account
        // claim with new account sends foodbar, pk and goes through the github authentication
        // to send auth token
        // existing account does the same thing but sends account name, account name is a noop
        if the transfer was performed user doesn't actually have to claim it it's done
        automatically this is checked prior to verification
        // API the verifier links the eternal user with the public key sent
        link_external_user(foodbar, pk:foodbar); // project_owner or verifier // replaces entry
        //if new account it transfers
        with external_entry



        transfer_external_user(foodbar,100);

         */
    }
}