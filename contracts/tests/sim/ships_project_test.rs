use near_sdk_sim::{to_yocto, call, view, DEFAULT_GAS, UserAccount};
use near_sdk::json_types::{Base58PublicKey};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::utils::init;
use near_sdk_sim::near_crypto::{InMemorySigner, KeyType, Signer};
use std::convert::{TryFrom, TryInto};
use near_sdk::serde_json::json;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use ships_project::{ProjectDetails, ProjectOrigin, ReleaseDetails, Version, ReleaseTerms};
use near_sdk::AccountId;

#[test]
fn simulate_some_view_function() {
  let (root, contract, _zane) = init();

  let actual: i8= view!(
    contract.get_count()
  ).unwrap_json();
  
  assert_eq!(9, actual);
  
}
#[test]
fn simulate_project_creation() {
    let (root, contract, _zane) = init();
    let bobby = root.create_user(AccountId::new_unchecked(String::from("bobby")), to_yocto("100"));

    let before_bobby = bobby.account().unwrap().amount;
    let before_contract = contract.account().unwrap().amount;
    let outcome = call!(
      bobby,
        contract.register_user(),
        deposit = to_yocto("0.00134")
    );
    outcome.assert_success();

    let outcome = call!(
      bobby,
        contract.create_project("first".to_string(),"https://github.com/myrepo".to_string(), ProjectDetails{
            org: "shipsgold".to_string(),
            origin_type: ProjectOrigin::Github,
            repo: "ships-contracts".to_string()
        }),
        deposit = to_yocto("0.00812")
    );
    outcome.assert_success();

    let outcome = call!(
      bobby,
        contract.create_new_release(1, ReleaseDetails {
            name: "Ships-Frontend".to_string(),
            version: Version {
                major: 0,
                minor: 0,
                patch: 1
            }
        },
        ReleaseTerms {
                max: 1000,
                min: 199,
                pre_allocation: 1000.into()
        }),
        deposit = to_yocto("0.00812")
    );
    let after_bobby = bobby.account().unwrap().amount;
    outcome.assert_success();

    let after_contract = contract.account().unwrap().amount;
    println!("contract: before {}", before_contract);
    println!("contract: after {}", after_contract);

    println!("bobby: before {}", before_bobby as f64/1e24);
    println!("bobby: diff {}", (before_bobby - after_bobby) as f64/ 1e24);
    println!("{:?}", outcome.gas_burnt());
    println!("{}", (outcome.tokens_burnt()) as f64 / 1e24);
}

#[test]
fn simulate_some_change_method() {
    let (root, contract, _zane) = init();

    let bobby = root.create_user(AccountId::new_unchecked(String::from("bobby")), to_yocto("1.0000000000000000000003"));
    let signer = InMemorySigner::from_seed("testuser", KeyType::ED25519, "testseed");
    let base_key = Base58PublicKey::try_from(bobby.signer.public_key().try_to_vec().unwrap()).unwrap();
    let result = call!(
          root,
          contract.add_guest(base_key)
      );

    assert!(result.is_ok());

    let result2 = call!(
      bobby,
      contract.increment()
    );
    assert!(result.is_ok());
}
