use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{near_bindgen, env, AccountId, Balance, PanicOnDefault, PromiseError, log, Gas, require, Promise};

pub mod external;
pub use crate::external::*;


const GAS_FOR_RESOLVE_CALLBACK: Gas = Gas(5_000_000_000_000);
const GAS_FOR_EXTERNAL_CALL: Gas = Gas(15_000_000_000_000 + GAS_FOR_RESOLVE_CALLBACK.0);

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub accounts: LookupMap<AccountId, Balance>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already Initialized");
        Self {accounts:LookupMap::new(b"a".to_vec())}
    }

    #[payable]
    pub fn deposit(&mut self, receiver_id: AccountId) {
        require!(env::prepaid_gas() > GAS_FOR_EXTERNAL_CALL, "More gas is required");
        let user = env::predecessor_account_id();
        let balance = env::attached_deposit();
        let new_balance = self.get_balance(&user) + balance;
        self.set_account(&user, &new_balance);

        re_entrancy::ext(receiver_id).with_static_gas(env::prepaid_gas() - GAS_FOR_EXTERNAL_CALL).reentrancey().then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_FOR_RESOLVE_CALLBACK)
                .deposit_callback(user, balance),
        );
    }

    #[private]
    pub fn deposit_callback(&mut self, #[callback_result]call_result: Result<String, PromiseError>, user: AccountId, balance: Balance){
        if call_result.is_err() {
            log!("There was an error contacting Callback");
            Promise::new(user.clone()).transfer(balance);
            let new_balance = self.get_balance(&user) - balance;
            self.set_account(&user, &new_balance);
        }

        log!("The Deposit was Successful")
    }

    pub fn balance_of(&self, account_id: AccountId) -> U128 {
        let balance = self.get_balance(&account_id);
        balance.into()
    }

    pub fn withdraw(&mut self) {
        let caller_id = env::predecessor_account_id();
        let amount = self.get_balance(&caller_id);

        Promise::new(caller_id.clone()).transfer(amount);

        self.set_account(&caller_id, &0);
    }
}

impl Contract {
    fn get_balance(&self, user: &AccountId) -> u128 {
        self.accounts.get(user).unwrap_or_else(|| 0)
    }

    fn set_account(&mut self, owner_id: &AccountId, balance: &Balance) {
        self.accounts.insert(&owner_id, &balance);
}
}