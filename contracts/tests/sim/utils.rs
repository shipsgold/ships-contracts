use near_sdk_sim::{deploy, init_simulator, to_yocto, STORAGE_AMOUNT, UserAccount, ContractAccount};
//use first_contract::Counter as Counter;
use counter::ContractContract as Counter;

const CONTRACT_ID: &str = "counter";

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
  // update `contract.wasm` for your contract's name
  //CONTRACT_WASM_BYTES => "target/wasm32-unknown-unknown/release/first_contract.wasm",

  // if you run `cargo build` without `--release` flag:
  CONTRACT_WASM_BYTES => "res/counter.wasm",

}

pub fn init() -> (UserAccount, ContractAccount<Counter>, UserAccount) {
  let root = init_simulator(None);

  let contract = deploy!(
      contract: Counter,
      contract_id: CONTRACT_ID,
      bytes: &CONTRACT_WASM_BYTES,
      signer_account: root,
      init_method: new(root.valid_account_id(),Some(8))
  );

  let  zane = root.create_user(
      "zane".to_string(),
      to_yocto("100000") // initial balance
  );

  (root, contract, zane)
}
