use near_sdk_sim::{call, view};
use crate::utils::init;

#[test]
fn simulate_some_view_function() {
  let (root, contract, _zane) = init();

  let actual: String = view!(
    contract.contract.get_count()
  ).unwrap_json();

  assert_eq!("0", actual);
}