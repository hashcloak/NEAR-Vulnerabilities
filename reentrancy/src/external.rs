use near_sdk::{ext_contract};

// Validator interface, for cross-contract calls
#[ext_contract(re_entrancy)]
trait ReEntrancy {
  fn reentrancey(&mut self);
}