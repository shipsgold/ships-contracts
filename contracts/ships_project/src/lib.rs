use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{PromiseOrValue, BorshStorageKey, ext_contract, Balance, PublicKey, env, near_bindgen, AccountId, PanicOnDefault, Promise, PromiseResult, StorageUsage};
use near_sdk::log;
use std::cmp::Ordering;
use multi_token_standard::metadata::{MultiTokenMetadataProvider, MultiTokenMetadata};
use multi_token_standard::{impl_multi_token_core, impl_multi_token_storage};


use near_sdk::collections::{LookupMap, LookupSet, TreeMap, Vector};

use std::collections::HashMap;

mod internal;
mod math;
mod account;
mod page;

use page::{PaginationOptions, PaginationResponse};
use crate::ReleaseStatus::ACTIVE;
use multi_token_standard::MultiToken;
use std::convert::TryFrom;

type CodeId = String;

type ProjectId = u64;
type ReleaseId = u64;
// sha256 hash of the project Details
type ProjectHash = Vec<u8>;

const ACCESS_KEY_ALLOWANCE: u128 = 100_000_000_000_000_000_000_000_000;

//const ACCESS_KEY_ALLOWANCE: u128 = 100_820_000_000_000_000_000_000;
#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Guests,
    OwnerToProjects,
    ProjectIdToProject { project_id: Vec<u8> },
    ProjectIdsToProjects,
    ProjectHashToProjectId,
    ProjectToReleaseIds,
    ProjectReleases { project_id: Vec<u8> },
    ReleaseIdToReleases,
    MultiTokenOwner,
    MultiTokenMetadata,
    MultiTokenSupply,
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
    details: ProjectDetails,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, PartialEq)]
#[cfg_attr(test, derive(Clone, Debug))]
pub struct Version {
    pub minor: u32,
    pub major: u32,
    pub patch: u32,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let major = self.major.cmp(&other.major);
        if major != Ordering::Equal {
            return Some(major);
        }
        let minor = self.minor.cmp(&other.minor);
        if minor != Ordering::Equal {
            return Some(minor);
        }
        return Some(self.patch.cmp(&other.patch));
    }
}

// This allows for the project details to change
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[cfg_attr(test, derive(Clone, Debug))]
pub struct ProjectDetails {
    pub repo: String,
    pub org: String,
    pub origin_type: ProjectOrigin,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[cfg_attr(test, derive(Clone, Debug))]
pub struct ReleaseDetails {
    pub name: String,
    pub version: Version,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[cfg_attr(test, derive(Clone, Debug))]
pub struct ReleaseTerms {
    pub min: u32,
    pub max: u32,
    pub pre_allocation: U128,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq)]
#[cfg_attr(test, derive(Clone, Debug))]
pub enum ReleaseStatus {
    ACTIVE,
    CLOSED,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[cfg_attr(test, derive(Clone, Debug))]
pub struct Release {
    version: Version,
    releaser: AccountId,
    name: String,
    min: u32,
    max: u32,
    release_id: ReleaseId,
    pre_allocation: Balance,
    status: ReleaseStatus,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    val: i8,
    token: MultiToken,
    guests: LookupSet<PublicKey>,
    owner_to_projects: LookupMap<String, Vector<ProjectId>>,
    project_hash_to_project_id: LookupMap<ProjectHash, ProjectId>,
    project_id_to_project: LookupMap<ProjectId, Project>,
    project_to_releases: LookupMap<ProjectId, Vector<ReleaseId>>,
    release_id_to_release: LookupMap<ReleaseId, Release>,
    project_storage_usage: u64,
    user_storage_usage: u64,
    guest_storage_usage: u64,
    release_storage_usage: u64,
    project_id_idx: u64,
    release_id_idx: u64,
    prefix_project_to_release_idx: u64,
    prefix_owner_to_projects_idx: u64,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(
            !env::state_exists(),
            "Contract has already been initialized"
        );

        let mut this = Self {
            owner: owner_id.clone(),
            val: 0,
            token: MultiToken::new(StorageKey::MultiTokenOwner,
                                   owner_id,
                                   Some(StorageKey::MultiTokenMetadata),
                                   StorageKey::MultiTokenSupply),
            guests: LookupSet::new(StorageKey::Guests),
            owner_to_projects: LookupMap::new(StorageKey::OwnerToProjects),
            project_hash_to_project_id: LookupMap::new(StorageKey::ProjectHashToProjectId),
            project_id_to_project: LookupMap::new(StorageKey::ProjectIdsToProjects),
            project_to_releases: LookupMap::new(StorageKey::ProjectToReleaseIds),
            release_id_to_release: LookupMap::new(StorageKey::ReleaseIdToReleases),
            project_storage_usage: 0,
            user_storage_usage: 0,
            guest_storage_usage: 0,
            release_storage_usage: 0,
            release_id_idx: 0,
            project_id_idx: 0,
            prefix_owner_to_projects_idx: 0,
            prefix_project_to_release_idx: 0,
        };
        this.measure_project_storage_usage();
        this.measure_user_storage_usage();
        this
    }

    fn inc_project_idx(&mut self) -> u64 {
        self.project_id_idx += 1;
        self.project_id_idx
    }

    fn inc_release_idx(&mut self) -> u64 {
        self.release_id_idx += 1;
        self.release_id_idx
    }

    fn inc_prefix_owner_to_projects(&mut self) -> u64 {
        self.prefix_owner_to_projects_idx += 1;
        self.prefix_owner_to_projects_idx
    }

    fn inc_prefix_project_to_release(&mut self) -> u64 {
        self.prefix_project_to_release_idx += 1;
        self.prefix_project_to_release_idx
    }

    fn measure_user_storage_usage(&mut self) {
        let initial_storage_usage = env::storage_usage();
        let tmp_owner_id = unsafe { String::from_utf8_unchecked(vec![b'a'; 64]) };
        let tmp_projects = Vector::new(
            StorageKey::ProjectIdToProject {
                project_id:
                self.prefix_owner_to_projects_idx.to_be_bytes().to_vec()
            });

        self.owner_to_projects.insert(&tmp_owner_id, &tmp_projects);
        let storage_usage = env::storage_usage();

        // cleanup
        self.owner_to_projects.remove(&tmp_owner_id);

        self.user_storage_usage = (storage_usage - initial_storage_usage);
    }

    fn min_guest_storage_cost(&mut self) {
        let initial_storage_usage = env::storage_usage();
        let tmp_guest_id = vec![b'a'; 64];
        let public_key  = PublicKey::try_from(tmp_guest_id).unwrap();
        self.guests.insert(&public_key);
        let storage_usage = env::storage_usage();
        self.guests.remove(&public_key);
        if self.user_storage_usage == 0 {
            self.measure_user_storage_usage();
        }
        self.guest_storage_usage = (storage_usage - initial_storage_usage).checked_add(self.user_storage_usage).unwrap();
    }

    fn calculate_project_hash(&self, project_details: &ProjectDetails) -> Vec<u8> {
        env::sha256(&project_details.try_to_vec().unwrap())
    }


    fn measure_project_storage_usage(&mut self) {
        let tmp_owner_id = unsafe { String::from_utf8_unchecked(vec![b'a'; 64]) };
        let tmp_details = unsafe { String::from_utf8_unchecked(vec![b'a'; 64]) };
        let tmp_uri = unsafe { String::from_utf8_unchecked(vec![b'u'; 250]) };
        let tmp_projects = Vector::new(
            StorageKey::ProjectIdToProject {
                project_id:
                self.prefix_owner_to_projects_idx.to_be_bytes().to_vec()
            });
        let tmp_releases = Vector::new(
            StorageKey::ProjectReleases {
                project_id:
                self.prefix_project_to_release_idx.to_be_bytes().to_vec()
            });


        let project_details = ProjectDetails {
            origin_type: ProjectOrigin::Github,
            org: tmp_details.clone(),
            repo: tmp_details.clone(),
        };

        let project = Project {
            id: self.project_id_idx,
            name: tmp_owner_id.to_string(),
            owner: AccountId::new_unchecked(tmp_owner_id.clone()),
            uri: tmp_uri.clone(),
            details: project_details,
        };
        self.owner_to_projects.insert(&project.owner.to_string(), &tmp_projects);
        let initial_storage_usage = env::storage_usage();
        self.owner_to_projects.get(&project.owner.to_string()).unwrap().push(&project.id);
        self.project_id_to_project.insert(&project.id, &project);
        let project_hash = self.calculate_project_hash(&project.details);
        self.project_hash_to_project_id.insert(&project_hash, &project.id);
        self.project_to_releases.insert(&project.id, &tmp_releases);
        let project_storage_usage = env::storage_usage();
        // clean up
        self.project_storage_usage = project_storage_usage - initial_storage_usage;

        let mut releases = self.project_to_releases.get(&project.id).unwrap();
        let release = Release {
            releaser: AccountId::new_unchecked(tmp_owner_id.clone()),
            release_id: self.inc_release_idx(),
            pre_allocation: 9u128,
            min: 10000,
            max: 20000,
            version: Version { major: 0, minor: 1, patch: 1 },
            name: tmp_details.clone(),
            status: ReleaseStatus::ACTIVE,
        };
        releases.push(&release.release_id);
        self.release_id_to_release.insert(&release.release_id, &release);
        self.project_to_releases.insert(&project.id, &releases);
        self.release_storage_usage = (env::storage_usage() - project_storage_usage);

        self.project_to_releases.remove(&project.id);
        self.owner_to_projects.remove(&project.owner.to_string());
        self.project_id_to_project.remove(&project.id);
        self.project_hash_to_project_id.remove(&project_hash);
    }

    #[payable]
    pub fn register_user(&mut self
    ) {
        let storage_cost = u128::from(self.user_storage_usage) * env::storage_byte_cost();
        let refund = env::attached_deposit().checked_sub(storage_cost)
            .unwrap_or_else(|| env::panic_str(format!("Project requires at least {} deposit", storage_cost).as_str()));
        let owner_id = env::predecessor_account_id();
        let tmp_projects = Vector::new(
            StorageKey::ProjectIdToProject {
                project_id:
                self.inc_prefix_owner_to_projects().to_be_bytes().to_vec()
            });
        self.owner_to_projects.insert(&owner_id.to_string(), &tmp_projects);
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }

    #[payable]
    pub fn create_project(&mut self, name: String, uri: String, details: ProjectDetails) {
        let owner_id = env::predecessor_account_id();
        let project_storage_cost = env::storage_byte_cost() * u128::from(self.project_storage_usage);
        let refund = env::attached_deposit().checked_sub(project_storage_cost)
            .unwrap_or_else(|| env::panic_str(format!("Project requires at least {} deposit", project_storage_cost).as_str()));
        let releases = Vector::new(
            StorageKey::ProjectReleases {
                project_id:
                self.inc_prefix_project_to_release().to_be_bytes().to_vec()
            });

        let project_hash = self.calculate_project_hash(&details);
        let project_id = self.inc_project_idx();
        if let Some(_id) = self.project_hash_to_project_id.insert(&project_hash, &project_id) {
            env::panic_str("This project already exists");
        }
        let project = Project {
            owner: owner_id,
            name,
            uri,
            id: project_id,
            details,
        };
        let mut projects = self.owner_to_projects.get(&project.owner.to_string()).unwrap();
        projects.push(&project_id);
        self.owner_to_projects.insert(&project.owner.to_string(), &projects);
        self.project_id_to_project.insert(&project.id, &project);
        self.project_to_releases.insert(&project.id, &releases);

        if refund > 0 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
        log!("project created {}", project.name);
    }

    pub fn get_project(&self, project_id: ProjectId) -> Option<Project> {
        self.project_id_to_project.get(&project_id)
    }

    pub fn get_projects(&self, owner_id: AccountId, options: Option<PaginationOptions>) -> Vec<ProjectId> {
        let projects = self.owner_to_projects.get(&owner_id.to_string()).unwrap();
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
    pub fn create_new_release(&mut self, project_id: ProjectId, details: ReleaseDetails, terms: ReleaseTerms) {
        let project = self.project_id_to_project.get(&project_id).unwrap_or_else(|| env::panic_str(format!("Project id: {} does not exist", project_id).as_str()));
        assert_eq!(project.owner, env::predecessor_account_id());

        // calculate refund
        let release_storage_cost = env::storage_byte_cost() * u128::from(self.release_storage_usage);
        let refund = env::attached_deposit().checked_sub(release_storage_cost)
            .unwrap_or_else(|| env::panic_str(format!("Project requires at least {} deposit", release_storage_cost).as_str()));

        // only callable by owner of the project
        let mut releases = self.project_to_releases.get(&project_id).unwrap();
        //verify version is increasing in nature and verify there are no active releases
        if releases.len() > 0 {
            let release_id = releases.get(releases.len() - 1).unwrap();
            let release = self.release_id_to_release.get(&release_id).unwrap();
            if release.status == ReleaseStatus::CLOSED {
                env::panic_str("Another release is active")
            }
            if release.version > details.version {
                env::panic_str(format!("Version is less than latest version {} {} {}",
                       release.version.major,
                       release.version.minor,
                       release.version.patch).as_str());
            }
        }
        let new_release = &Release {
            name: details.name,
            version: details.version,
            max: terms.max,
            min: terms.min,
            pre_allocation: terms.pre_allocation.into(),
            release_id: self.inc_release_idx(),
            releaser: env::predecessor_account_id(),
            status: ACTIVE,
        };
        self.release_id_to_release.insert(&new_release.release_id, &new_release);
        releases.push(&new_release.release_id);
        self.project_to_releases.insert(&project_id, &releases);
        self.internal_mint_release_token(&new_release.release_id, 1000);
    }

    pub fn get_release(&self, release_id: ReleaseId) -> Option<Release> {
        self.release_id_to_release.get(&release_id)
    }

    pub fn get_releases(&self, project_id: ProjectId, options: Option<PaginationOptions>) -> Vec<Release> {
        let releases = self.project_to_releases.get(&project_id).unwrap();
        let opt = options.unwrap_or_default();
        let mut range = (opt.from..std::cmp::min(opt.from + opt.limit, releases.len()));

        if opt.reverse {
            let from = std::cmp::min(opt.from - opt.limit, 0);
            range = (from..std::cmp::min(opt.from, releases.len()));
        }
        range
            .map(|index| self.release_id_to_release.get(&releases.get(index).unwrap()).unwrap())
            .collect()
    }

    fn internal_get_release_token_id(&self, release_id: &ReleaseId) -> multi_token_standard::TokenId {
        format!("{:x}", release_id)
    }


    fn internal_mint_release(&mut self, owner_id: &AccountId, token_id: &multi_token_standard::TokenId, token_type: multi_token_standard::TokenType, amount: Option<u128>) {
        let initial_storage_usage = env::storage_usage();


        // Every token must have a token type and every NFT type cannot be reminted
        match self.token.token_type_index.get(token_id) {
            Some(TokenType::Ft) => {
                assert_eq!(token_type, TokenType::Ft, "Type must be of FT time tokenId already exists")
            }
            Some(TokenType::Nft) => {
                env::panic_str("Attempting to mint already minted NFT");
            }
            None => {
                self.token.token_type_index.insert(token_id, &token_type);
            }
        }

        // Core behavior: every token must have an owner
        match token_type {
            TokenType::Ft => {
                if amount.is_none() {
                    env::panic_str("Amount must be specified for Ft type tokens");
                }
                // advance the prefix index before insertion
                let amt = u128::from(amount.unwrap());
                //create TreeMap for balances
                match self.token.ft_owners_by_id.get(&token_id) {
                    Some(mut balances) => {
                        let current_bal = balances.get(&owner_id).unwrap_or(0);
                        // TODO not quite safe
                        if amt == 0 {
                            panic!("error: amount should be greater than 0")
                        }
                        balances.insert(&owner_id, &(current_bal + amt));
                        let supply = self.token.ft_token_supply_by_id.get(&token_id).unwrap();
                        self.token.ft_token_supply_by_id.insert(&token_id, &(supply + amt));
                    }
                    None => {
                        let mut balances = self.token.internal_new_ft_balances();
                        // insert amount into balances
                        balances.insert(&owner_id, &amt);
                        self.token.ft_owners_by_id.insert(&token_id, &balances);
                        self.token.ft_token_supply_by_id.insert(&token_id, &amt);
                    }
                }
            }
            TokenType::Nft => {
                self.token.nft_owner_by_id.insert(&token_id, &owner_id);
            }
        }

        // Metadata extension: Save metadata, keep variable around to return later.
        // Note that check above already panicked if metadata extension in use but no metadata
        // provided to call.
        /*
        self.token
        .token_metadata_by_id
        .as_mut()
        .and_then(|by_id| by_id.insert(&token_id, &metadata.as_ref().unwrap()));
        */

        // Return any extra attached deposit not used for storage
        self.internal_refund_deposit(env::storage_usage() - initial_storage_usage);
    }

    fn internal_refund_deposit(&self, storage_used: u64) {
        let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
        let attached_deposit = env::attached_deposit();
        assert!(
            required_cost <= attached_deposit,
            "Must attach {} yoctoNEAR to cover storage",
            required_cost,
        );
        let refund = attached_deposit - required_cost;
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }


    fn internal_mint_release_token(&mut self, release_id: &ReleaseId, amount: u128) {
        let token_id = self.internal_get_release_token_id(release_id);
        self.internal_mint_release(&env::predecessor_account_id(), &token_id, multi_token_standard::TokenType::Ft, Some(amount));
        // TODO valid accountId restriction on minting shouldn't exist
        /*self.token.mint("1".into(), TokenType::Ft, Some(amount.into()), ValidAccountId::from(ValidAccountId::try_from(env::predecessor_account_id()).unwrap()), Some(MultiTokenMetadata{
            spec: "".to_string(),
            name: "".to_string(),
            symbol: "".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None
        }))*/
    }

    pub fn get_token_id(&self, release_id: U64) -> String {
        format!("{:x}", u64::from(release_id))
    }

    /// TODO temporary to test access patterns
    #[payable]
    pub fn increment(&mut self) -> i8 {
        self.val = self.val + 1;
        self.val
    }
    pub fn get_count(&self) -> i8 {
        9
    }
}


impl multi_token_standard::metadata::MultiTokenMetadataProvider for Contract {
    fn mt_metadata(&self, token_id: TokenId) -> MultiTokenMetadata {
        // TODO this conversion might be tricky and need padding so it's consistent
        let release_id = u64::from_str_radix(&token_id, 16).unwrap();
        let release = self.release_id_to_release.get(&release_id).unwrap();
        // TODO will need a scheme for storing this data
        // TODO symbol needs thought
        MultiTokenMetadata {
            decimals: 18.into(),
            name: release.name,
            base_uri: None,
            icon: None,
            reference: None,
            reference_hash: None,
            title: None,
            description: None,
            media: None,
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            spec: multi_token_standard::metadata::MT_METADATA_SPEC.into(),
            symbol: "TTTT".to_string(),
            extra: None,
        }
    }
}

multi_token_standard::impl_multi_token_core!(Contract, token);
multi_token_standard::impl_multi_token_storage!(Contract, token);


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

    fn get_context(current_id: AccountId,
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

    fn get_contract_id() -> AccountId {
        accounts(1)
    }

    fn get_sponsor() -> AccountId {
        accounts(0)
    }

    fn get_sponsor_pk() -> PublicKey {
        PublicKey::try_from(vec![1, 2, 3]).unwrap()
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
        let base_key = PublicKey::try_from(signer.public_key().try_to_vec().unwrap()).unwrap();
        println!("{}", String::try_from(&base_key).unwrap());
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1));
        contract.register_user();
        contract.create_project("foobar".to_string(), "https://foobar".to_string(), ProjectDetails {
            org: "shipsgold".to_string(),
            origin_type: ProjectOrigin::Github,
            repo: "ships-contract".to_string(),
        });
        let project = contract.get_project(1);
        let projects = contract.get_projects(AccountId::new_unchecked("bob".to_string()), None);
        contract.create_new_release(1, ReleaseDetails {
            version: Version {
                major: 1,
                minor: 0,
                patch: 0,
            },
            name: "EdgyEgret".to_string(),
        },
                                    ReleaseTerms {
                                        min: 10000,
                                        max: 20000,
                                        pre_allocation: 100.into(),
                                    });
        let release = contract.get_release(1).unwrap();
        println!("{:?}", release);
        let releases = contract.get_releases(1, None);
        println!("{:?}", releases);
        let token_id = contract.get_token_id(release.release_id.into());
        println!("{}", token_id);
        //contract.balance_of_batch(env::predecessor_account_id(),vec!["1".to_string()]);

        // TODO
        let t = format!("{:x}", 1);
        println!("{}", u128::from(contract.balance_of(env::predecessor_account_id(), t)));

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
