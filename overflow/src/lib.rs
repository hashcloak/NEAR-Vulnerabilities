use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    counter: u8,
}

#[near_bindgen]
impl Contract {
    pub fn view_counter(&self) -> u8 {
        self.counter
    }

    pub fn update_counter(&mut self, number: u8) {
        self.counter += number;
    }

    pub fn update_counter_by_1(&mut self) {
        self.counter += 1;
    }
}