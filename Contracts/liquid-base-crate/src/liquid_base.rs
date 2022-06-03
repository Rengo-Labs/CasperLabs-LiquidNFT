use crate::data::{self, Compensations, Contributions};
use casper_types::{Key, U256};
use common::keys::zero_address;
use contract_utils::{get_key, set_key};
use contract_utils::{ContractContext, ContractStorage};

#[allow(non_snake_case)]
pub trait LIQUIDBASE<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self) {
        data::Compensations::init();
        data::Contributions::init();
    }

    // Mappings
    fn Compensations(&self) -> Compensations {
        Compensations::instance()
    }

    fn Contributions(&self) -> Contributions {
        Contributions::instance()
    }

    // Variables
    fn set_single_provider(&self, single_provider: Key) {
        set_key(data::SINGLE_PROVIDER, single_provider);
    }

    fn get_single_provider(&self) -> Key {
        get_key(data::SINGLE_PROVIDER).unwrap_or(zero_address())
    }

    //Minimum the owner wants for the loan. If less than this contributors refunded
    fn set_floor_asked(&self, floor_asked: U256) {
        set_key(data::FLOOR_ASKED, floor_asked);
    }

    fn get_floor_asked(&self) -> U256 {
        get_key(data::FLOOR_ASKED).unwrap_or_default()
    }

    //Maximum the owner wants for the loan
    fn set_total_asked(&self, total_asked: U256) {
        set_key(data::TOTAL_ASKED, total_asked);
    }

    fn get_total_asked(&self) -> U256 {
        get_key(data::TOTAL_ASKED).unwrap_or_default()
    }

    //How many tokens have been collected for far for this loan
    fn set_total_collected(&self, total_collected: U256) {
        set_key(data::TOTAL_COLLECTED, total_collected);
    }

    fn get_total_collected(&self) -> U256 {
        get_key(data::TOTAL_COLLECTED).unwrap_or_default()
    }

    //Balance contributors can claim at a given moment
    fn set_claimable_balance(&self, claimable_balance: U256) {
        set_key(data::CLAIMABLE_BALANCE, claimable_balance);
    }

    fn get_claimable_balance(&self) -> U256 {
        get_key(data::CLAIMABLE_BALANCE).unwrap_or_default()
    }

    //Balance the locker owner still owes
    fn set_remaining_balance(&self, remaining_balance: U256) {
        set_key(data::REMAINING_BALANCE, remaining_balance);
    }

    fn get_remaining_balance(&self) -> U256 {
        get_key(data::REMAINING_BALANCE).unwrap_or_default()
    }

    //Balance of all penalties incurred by locker owner so far
    fn set_penalties_balance(&self, penalties_balance: U256) {
        set_key(data::PENALTIES_BALANCE, penalties_balance);
    }

    fn get_penalties_balance(&self) -> U256 {
        get_key(data::PENALTIES_BALANCE).unwrap_or_default()
    }

    //Time next payoff must happen to avoid penalties
    fn set_next_due_time(&self, next_due_time: U256) {
        set_key(data::NEXT_DUE_TIME, next_due_time);
    }

    fn get_next_due_time(&self) -> U256 {
        get_key(data::NEXT_DUE_TIME).unwrap_or_default()
    }

    //Timestamp initialize was called
    fn set_creation_time(&self, creation_time: U256) {
        set_key(data::CREATION_TIME, creation_time);
    }

    fn get_creation_time(&self) -> U256 {
        get_key(data::CREATION_TIME).unwrap_or_default()
    }
}
