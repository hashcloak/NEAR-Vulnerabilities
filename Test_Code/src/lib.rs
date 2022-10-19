use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    called: u128
}

#[near_bindgen]
impl Contract {
    pub fn reentrancy(&mut self) {
        self.called += 1;
    }

    pub fn called_status(&self) -> u128 {
        self.called
    }
}