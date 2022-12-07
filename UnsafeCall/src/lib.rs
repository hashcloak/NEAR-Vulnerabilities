use near_sdk::{near_bindgen, AccountId, env, ext_contract, PromiseOrValue};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[ext_contract(ext_ft_receiver)]
pub trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl Contract {
    pub fn query_greeting(receiver_id: AccountId, amount: U128, msg: String) {
        let sender_id = env::predecessor_account_id();
        ext_ft_receiver::ext(receiver_id.clone())
            .with_static_gas(env::prepaid_gas())
            .ft_on_transfer(sender_id.clone(), amount, msg);
    }
}