use near_sdk_sim::{deploy, init_simulator, to_yocto, STORAGE_AMOUNT, UserAccount, ContractAccount};
//use first_contract::Counter as Counter;
use ships_project::ContractContract as ShipsProject;
use near_sdk::AccountId;

const CONTRACT_ID: &str = "ships_project";

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
  // update `contract.wasm` for your contract's name
  //CONTRACT_WASM_BYTES => "target/wasm32-unknown-unknown/release/first_contract.wasm",

  // if you run `cargo build` without `--release` flag:
  CONTRACT_WASM_BYTES => "res/ships_project.wasm",

}

pub fn init() -> (UserAccount, ContractAccount<ShipsProject>, UserAccount) {
  let root = init_simulator(None);
    println!("{}", CONTRACT_WASM_BYTES.len());


  let contract = deploy!(
      contract: ShipsProject,
      contract_id: CONTRACT_ID,
      bytes: &CONTRACT_WASM_BYTES,
      signer_account: root,
      init_method: new(root.account_id())
  );



  let  zane = root.create_user(
      AccountId::new_unchecked("zane".to_string()),
      to_yocto("100000") // initial balance
  );

  (root, contract, zane)
}
