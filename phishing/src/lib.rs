use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, env, PanicOnDefault, Balance, Promise, require};

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_address: AccountId) -> Self {
        assert!(!env::state_exists(), "Already Initialized");
        Self {owner: owner_address}
    }

    pub fn transfer(&self, account_id: AccountId, balance: Balance) {
        require!(env::signer_account_id() == self.owner, "Not owner");
        Promise::new(account_id).transfer(balance);
    }
}
