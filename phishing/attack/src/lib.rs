use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, ext_contract, AccountId, Balance};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

// phishing interface, for cross-contract calls
#[ext_contract(phishing)]
trait PhishingExt {
  fn transfer(&self, account_id: AccountId, balance: Balance);
}

#[near_bindgen]
impl Contract {
    pub fn callback_transfer(&self, receiver_id: AccountId, caller_id: AccountId, balance: Balance) {
        phishing::ext(receiver_id.clone()).transfer(caller_id, balance);
    }
}
