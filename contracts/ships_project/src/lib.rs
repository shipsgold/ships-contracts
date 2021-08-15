use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128, Base58PublicKey};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{BorshStorageKey, ext_contract, Balance, PublicKey, env, near_bindgen, setup_alloc, AccountId, PanicOnDefault, Promise, PromiseResult, StorageUsage};
use near_sdk::log;
use multi_token_standard::metadata::{MultiTokenMetadata};



use near_sdk::collections::{LookupMap,LookupSet, TreeMap, Vector};

use std::collections::HashMap;
mod internal;
mod math;
mod account;
mod page;

use page::{PaginationOptions,PaginationResponse};
type CodeId = String;
setup_alloc!();

type ProjectId = u64;
type ReleaseId = u64;
// sha256 hash of the project Details
type ProjectHash = Vec<u8>;

const ACCESS_KEY_ALLOWANCE: u128 = 100_000_000_000_000_000_000_000_000;
//const ACCESS_KEY_ALLOWANCE: u128 = 100_820_000_000_000_000_000_000;
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Guests,
    OwnerToProjects,
    ProjectIdToProject { project_id: Vec<u8> },
    ProjectIdsToProjects,
    ProjectHashToProjectId,
    ProjectToReleases,
    ProjectReleases {project_id: Vec<u8>}
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Debug))]
pub enum ProjectOrigin {
    Github
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[cfg_attr(test, derive(Clone, Debug))]
pub struct Project {
  name: String,
  owner: AccountId,
  uri: String,
  id: ProjectId, 
  details: ProjectDetails
}
// This allows for the project details to change
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[cfg_attr(test, derive(Clone, Debug))]
pub struct ProjectDetails {
  repo: String,
  org: String,
  origin_type: ProjectOrigin 
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[cfg_attr(test, derive(Clone, Debug))]
pub struct Release {
  version: String,
  releaser: AccountId,
  name: String,
  min: u32,
  max: u32,
  release_id: ReleaseId, 
  pre_allocation: Balance
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: ValidAccountId,
    val: i8,
    guests: LookupSet<PublicKey>,
    owner_to_projects: LookupMap<String, Vector<ProjectId>>,
    project_hash_to_project_id: LookupMap<ProjectHash, ProjectId>,
    project_id_to_project: LookupMap<ProjectId, Project>,
    project_to_releases: LookupMap<ProjectId, Vector<Release>>,
    project_storage_cost: Balance,
    user_storage_cost: Balance,
    project_id_idx: u64,
    release_id_idx: u64,
    prefix_project_to_release_idx: u64,
    prefix_owner_to_projects_idx: u64,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId) -> Self {
        assert!(
            !env::state_exists(),
            "Contract has already been initialized"
        );

        let mut this = Self {
            owner: owner_id,
            val: 0,
            guests: LookupSet::new(StorageKey::Guests),
            owner_to_projects: LookupMap::new(StorageKey::OwnerToProjects),
            project_hash_to_project_id: LookupMap::new(StorageKey::ProjectHashToProjectId),
            project_id_to_project: LookupMap::new(StorageKey::ProjectIdsToProjects),
            project_to_releases: LookupMap::new(StorageKey::ProjectToReleases),
            project_storage_cost: 0,
            user_storage_cost: 0,
            release_id_idx: 0,
            project_id_idx: 0,
            prefix_owner_to_projects_idx: 0,
            prefix_project_to_release_idx: 0
        };
        this.project_storage_cost = this.min_project_storage_cost();
        this.user_storage_cost = this.min_user_storage_cost();
        this
    }

    fn inc_project_idx(&mut self) -> u64 {
        self.project_id_idx +=1;
        self.project_id_idx
    }

    fn inc_release_idx(&mut self) -> u64 {
        self.release_id_idx +=1;
        self.release_id_idx
    }

    fn inc_prefix_owner_to_projects(&mut self) -> u64 {
        self.prefix_owner_to_projects_idx +=1;
        self.prefix_owner_to_projects_idx
    }

    fn inc_prefix_project_to_release(&mut self) -> u64 {
        self.prefix_project_to_release_idx +=1;
        self.prefix_project_to_release_idx
    }

    fn min_user_storage_cost(&mut self) -> u128 {
        let initial_storage_usage = env::storage_usage();
        let tmp_owner_id = unsafe { String::from_utf8_unchecked(vec![b'a'; 64]) };
        let tmp_projects = Vector::new(
           StorageKey::ProjectIdToProject{project_id: 
            self.prefix_owner_to_projects_idx.to_be_bytes().to_vec()
       });

        self.owner_to_projects.insert(&tmp_owner_id, &tmp_projects);
        let storage_usage = env::storage_usage();

        // cleanup
        self.owner_to_projects.remove(&tmp_owner_id);

        u128::from(storage_usage - initial_storage_usage) * env::storage_byte_cost()
    }

    fn min_guest_storage_cost(&mut self) -> u128 {
        let initial_storage_usage = env::storage_usage();
        let tmp_guest_id = vec![b'a'; 64];
        self.guests.insert(&tmp_guest_id);
        let storage_usage = env::storage_usage();
        self.guests.remove(&tmp_guest_id);
        let storage_used:u128 = (storage_usage - initial_storage_usage).into();
        let cost = storage_used * env::storage_byte_cost();
        cost.checked_add(self.min_user_storage_cost()).unwrap()
    }

    fn calculate_project_hash(&self, project_details: &ProjectDetails) ->Vec<u8> {
        env::sha256(&project_details.try_to_vec().unwrap())      
    }

    fn min_project_storage_cost(&mut self) -> u128{
        let tmp_owner_id = unsafe { String::from_utf8_unchecked(vec![b'a'; 64]) };
        let tmp_details = unsafe { String::from_utf8_unchecked(vec![b'a'; 64]) };
        let tmp_uri= unsafe { String::from_utf8_unchecked(vec![b'u'; 250]) };
        let tmp_projects = Vector::new(
           StorageKey::ProjectIdToProject{project_id: 
            self.prefix_owner_to_projects_idx.to_be_bytes().to_vec()
        });


        let project_details = ProjectDetails {
            origin_type: ProjectOrigin::Github,
            org: tmp_details.clone(),
            repo: tmp_details.clone(),
        };

        let project =  Project {
            id: self.project_id_idx,
            name: tmp_owner_id.to_string(),
            owner: tmp_owner_id,
            uri: tmp_uri.clone(), 
            details: project_details
        };
        self.owner_to_projects.insert(&project.owner, &tmp_projects);
        let initial_storage_usage = env::storage_usage();
        self.owner_to_projects.get(&project.owner).unwrap().push(&project.id);
        self.project_id_to_project.insert(&project.id, &project);
        let project_hash = self.calculate_project_hash(&project.details);
        self.project_hash_to_project_id.insert(&project_hash, &project.id);
        let storage_usage = env::storage_usage();
        // clean up
        self.owner_to_projects.remove(&project.owner);
        self.project_id_to_project.remove(&project.id);
        self.project_hash_to_project_id.remove(&project_hash);
        u128::from(storage_usage - initial_storage_usage) * env::storage_byte_cost()
    }

    #[payable]
    pub fn register_user(&mut self) {
        let refund = env::attached_deposit().checked_sub(self.user_storage_cost)
        .unwrap_or_else(|| panic!("Project requires at least {} deposit",self.user_storage_cost));   
        let owner_id = env::predecessor_account_id();
        let tmp_projects = Vector::new(
           StorageKey::ProjectIdToProject{project_id: 
            self.prefix_owner_to_projects_idx.to_be_bytes().to_vec()
        });
        self.owner_to_projects.insert(&owner_id, &tmp_projects);
    }

    #[payable]
    pub fn create_project(&mut self, name: String, uri: String, details: ProjectDetails) {
        let owner_id = env::predecessor_account_id();
        let refund = env::attached_deposit().checked_sub(self.project_storage_cost)
        .unwrap_or_else(|| panic!("Project requires at least {} deposit",self.project_storage_cost));   

        let project_hash = self.calculate_project_hash(&details);
        let project_id = self.inc_project_idx();
        if let Some(_id) = self.project_hash_to_project_id.insert(&project_hash, &project_id) {
            panic!("This project already exists");
        }
        let project = Project {
            owner: owner_id,
            name,
            uri,
            id: project_id,
            details
        };
        let mut projects = self.owner_to_projects.get(&project.owner).unwrap();
        projects.push(&project_id);
        self.owner_to_projects.insert(&project.owner, &projects);
        self.project_id_to_project.insert(&project.id, &project);

        if refund > 0 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
        log!("project created {}", project.name);
        
    }

    pub fn get_project(&self, project_id: ProjectId) -> Option<Project>{
        self.project_id_to_project.get(&project_id)
    }

    pub fn get_projects(&self, owner_id: AccountId, options: Option<PaginationOptions>) -> Vec<ProjectId>{
        let projects = self.owner_to_projects.get(&owner_id).unwrap();
        let opt = options.unwrap_or_default();
        let mut range = (opt.from..std::cmp::min(opt.from + opt.limit, projects.len()));

        if opt.reverse {
            let from = std::cmp::min(opt.from - opt.limit, 0);
            range = (from..std::cmp::min(opt.from, projects.len()));
        }
            range
            .map(|index| projects.get(index).unwrap())
            .collect()
    }

    #[payable]
    pub fn create_new_release(&mut self, project_id: String,  version: String){


    }

    #[payable]
    pub fn mint_release_token(project_name: String){


    }

    /// TODO temporary to test access patterns
    #[payable]
    pub fn increment(&mut self) -> i8{
        self.val = self.val + 1; 
        self.val
    }
    pub fn get_count(&self) -> i8 {
         9
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
            .attached_deposit(13400000000000000000000)
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
        let contract = Contract::new(get_sponsor().into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.get_count(), 9);
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
        let mut contract = Contract::new(accounts(1));
        contract.register_user();
        contract.create_project("foobar".to_string(), "https://foobar".to_string(), ProjectDetails{
            org: "shipsgold".to_string(),
            origin_type: ProjectOrigin::Github,
            repo: "ships-contract".to_string(),
        });
        let project = contract.get_project(1);
        let projects = contract.get_projects("bob".to_string(), None);
        println!("{:?}", project);
/*        let mut contract = Contract {
            owner: accounts(1),
            val: 0,
            guests: LookupSet::new(StorageKey::Guests),

            
        };
        contract.add_guest(base_key.clone(),String::from("funnyuser"));
        let user = contract.guest_code_id(base_key.clone());
        assert_eq!(user, "funnyuser".to_string());
        let spawned_user = ValidAccountId::try_from(String::from("sponsored_user")).unwrap();
        context = get_context(spawned_user.clone(),
        spawned_user, 
        ValidAccountId::try_from(String::from("sponsored_user")).unwrap(),
        vec![9,9,9,9,9]);
        testing_env!(context.build());
        contract.increment();
        println!("Value after increment: {}", contract.get_count());
        assert_eq!(9, contract.get_count());
        */
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
