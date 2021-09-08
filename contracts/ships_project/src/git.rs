use near_sdk::env;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use crate::*;

const DAY_NANOSECONDS:u64 = 86_400_000_000_000;
impl Contract {
    /*
     Functionality here allows for the transfer of ext tokens to place holder external user system
     the system requirements are that per project , the username is unique per external system
     rules. If this is the case it's easy to represent a system like git that has an external
     verifier. This allows projects to outsource the verification of users to external systems
     In the case of git,

    */


    /// Function is used to send funds to the specified user it sends tokens to the temporary user
    /// assuming that the temporary user will link the account this allows the developer to simply
    /// send a link that can point a user to page to call the link_ext_user functionality, which will
    /// complete the final transfer, by removing the existing entry and writing a new one reclaiming
    /// the storage spent on the registering of the user as funds get transferred to the verifying account
    /// when the account the verifier triggers a transfer. The verifier is always a source of potential
    /// foul play as the verifier could "fraud". Ideally verification only happens for small amounts,
    /// and users create either temporary accounts taht are then used in smart contract lookup instead of
    /// transfer so ths transfer functionality goes directly to the registered or linked account.
    pub fn transfer_ext_user(&mut self, release_id: ReleaseId, user: String, amount: U128){
        let token_id = self.internal_get_release_token_id(&release_id);
        let user_token_id = EXT_USER_PREFIX.to_owned() + &token_id;
        // First we burn which will panic if there's not enough funds for it
        self.internal_burn_release_token(&token_id, amount.into());
        // Then we transfer to the user version
        self.internal_mint_release_unguarded(&AccountId::new_unchecked(user), &user_token_id, multi_token_standard::TokenType::Ft, Some(amount.into()));
    }

    pub fn verify_ext_user(&mut self, project_id: ProjectId, user:String, user_id:AccountId){
        // NOTE here that the project owner can verify, this is assumed to be temporary
        // pending claims, so the idea is that you'll claim it before it builds up to
        // anything substantial
        require!(env::predecessor_account_id() == self.verifier ||
            env::predecessor_account_id() == self.checked_get_project(&project_id).owner, "not project owner or verifier");
        let mut verification_status = self.project_id_to_ext_users.get(&project_id)
            .unwrap();
        verification_status.insert(&user, &ExtProjectUserStatus {
            block_timestamp: env::block_timestamp(),
            user_id
        });
        self.project_id_to_ext_users.insert(&project_id, &verification_status);
    }

    // NOTE this mus make sure that for supply we count both sets of supply
    pub fn claim_ext_user_tokens(&mut self, project_id: ProjectId, token_id: TokenId, user: String){
       let user_token_id = EXT_USER_PREFIX.to_owned() + &token_id;
       let verifications = self.project_id_to_ext_users
           .get(&project_id).unwrap();
        let status = verifications.get(&user).unwrap();
        require!(status.user_id == env::predecessor_account_id(),
            format!("{} Does not match accountid {}", env::predecessor_account_id(), status.user_id));
        let time_diff = env::block_timestamp().checked_sub(status.block_timestamp).unwrap();
        require!(time_diff < DAY_NANOSECONDS);
        // get current amount of value from the token
        let amount = self.token.internal_unwrap_balance_of(&user_token_id,&AccountId::new_unchecked(user.into()));
        // burn that  value and mint for free the equivalent tokens
        self.internal_burn_release_token(&user_token_id, amount);
        // mints the equivalent amount of tokens from ext users token to the main token id
        self.internal_mint_release_unguarded(&env::predecessor_account_id(), &token_id, TokenType::Ft, Some(amount));
    }

}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    use near_sdk::{testing_env, VMContext, PublicKey};
    use test_utils::*;
    #[test]
    pub fn basic_test() {
        let mut context = get_context(get_sponsor(),
                                      get_contract_id(),
                                      get_sponsor(),
                                      get_sponsor_pk());
        testing_env!(context.build());

        let mut contract = Contract::new(get_sponsor().into(), get_sponsor().into());
        contract.register_user();
        let project_id = contract.create_project("test".to_string(), "test".to_string(), ProjectDetails{
            repo: "test".to_string(),
            origin_type: ProjectOrigin::Github,
            org:"shipsgold".to_string()
        });
        contract.create_new_release(project_id.into(), ReleaseDetails {
            name: "".to_string(),
            version: Version {
                major: 0,
                minor: 0,
                patch: 1
            }
        }, ReleaseTerms {
            min: 0,
            max: 0,
            pre_allocation: U128(1000)
        });
        let releases = contract.get_releases(project_id.into(),None);
        let release = releases.get(0).unwrap();
        // contract.transfer_ext_user(release.release_id, "ext_user".to_string(), 100.into());
        //contract.balance_of(get_sponsor(),)
        //contract.register_ext_user(project_id.into(), "testuser".to_string(), "tmpid".to_string());
        //let user_id = contract.get_registered_user(project_id.into(), "testuser".to_string());
     //   assert_eq!(user_id.unwrap(),"tmpid".to_string());
        //let user_id = contract.get_registered_user(project_id.into(), "test".to_string());
       // assert_eq!(user_id, None);

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