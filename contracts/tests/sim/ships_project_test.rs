use near_sdk_sim::{to_yocto, call, view, DEFAULT_GAS, UserAccount};
use near_sdk::json_types::{Base58PublicKey};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::utils::init;
use near_sdk_sim::near_crypto::{InMemorySigner, KeyType, Signer};
use std::convert::{TryFrom, TryInto};
use near_sdk::serde_json::json;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[test]
fn simulate_some_view_function() {
  let (root, contract, _zane) = init();

  let actual: i8= view!(
    contract.get_count()
  ).unwrap_json();
  


  assert_eq!(9, actual);
  
}

#[test]
fn simulate_some_change_method() {
    let (root, contract, _zane) = init();

    let bobby = root.create_user(String::from("bobby"), to_yocto("1.0000000000000000000003"));
    let signer = InMemorySigner::from_seed("testuser", KeyType::ED25519, "testseed");
    let base_key = Base58PublicKey::try_from(bobby.signer.public_key().try_to_vec().unwrap()).unwrap();
    let result = call!(
          root,
          contract.add_guest(base_key, "funnyuser_id".to_string())
      );

    assert!(result.is_ok());

    let result2 = call!(
      bobby,
      contract.increment()
    );
    assert!(result.is_ok());
}
