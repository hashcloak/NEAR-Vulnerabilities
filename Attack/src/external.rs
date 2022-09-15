use near_sdk::{ext_contract, AccountId};

// Validator interface, for cross-contract calls
#[ext_contract(re_entrancy)]
trait ReEntrancy {
  fn deposit(&mut self, receiver_id: AccountId);
  fn withdraw(&mut self);
}