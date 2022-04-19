use crate::data::{self, Compensations, Contributions, Globals};
use alloc::string::ToString;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{Key, U256};
use contract_utils::{get_key, set_key};
use contract_utils::{ContractContext, ContractStorage};

pub trait LIQUIDBASE<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self) {
        data::Compensations::init();
        data::Contributions::init();
        data::Globals::init();
        self.set_payment_token(self.get_caller());
        self.set_creation_time(0.into());
        self.set_next_due_time(0.into());
        self.set_total_collected(0.into());
        self.set_claimable_balance(0.into());
        self.set_floor_asked(0.into());
        self.set_penalties_balance(0.into());
        self.set_remaining_balance(0.into());
        self.set_single_provider(self.ZERO_ADDRESS());
    }

    fn get_factory_address(&self) -> Key {
        get_key(data::FACTORY_ADDRESS).unwrap_or_revert()
    }
    fn set_factory_address(&self,factory_address:Key){
        set_key(data::FACTORY_ADDRESS, factory_address);
    }

    fn get_trustee_multisig(&self) -> Key {
        get_key(data::TRUSTEE_MULTISIG).unwrap_or_revert()
    }
    fn set_trustee_multisig(&self,trustee_multisig:Key){
        set_key(data::TRUSTEE_MULTISIG, trustee_multisig);
    }
    fn get_payment_token(&self) -> Key {
        get_key(data::PAYMENT_TOKEN).unwrap_or_revert()
    }
    fn set_payment_token(&self,payment_token:Key){
        set_key(data::PAYMENT_TOKEN, payment_token);
    }
    fn PAYMENT_TOKEN(&self) -> Key {
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap()
    }

    fn ZERO_ADDRESS(&self) -> Key {
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap()
    }

    // Mappings
    fn Compensations(&self) -> Compensations {
        Compensations::instance()
    }

    fn Contributions(&self) -> Contributions {
        Contributions::instance()
    }

    fn Globals(&self) -> Globals {
        Globals::instance()
    }

    // Variables
    fn set_single_provider(&self, single_provider: Key) {
        set_key(data::SINGLE_PROVIDER, single_provider);
    }

    fn get_single_provider(&self) -> Key {
        get_key(data::SINGLE_PROVIDER).unwrap_or_revert()
    }

    //Minimum the owner wants for the loan. If less than this contributors refunded
    fn set_floor_asked(&self, floor_asked: U256) {
        set_key(data::FLOOR_ASKED, floor_asked);
    }

    fn get_floor_asked(&self) -> U256 {
        get_key(data::FLOOR_ASKED).unwrap_or_revert()
    }

    //Maximum the owner wants for the loan
    fn set_total_asked(&self, total_asked: U256) {
        set_key(data::TOTAL_ASKED, total_asked);
    }

    fn get_total_asked(&self) -> U256 {
        get_key(data::TOTAL_ASKED).unwrap_or_revert()
    }

    //How many tokens have been collected for far for this loan
    fn set_total_collected(&self, total_collected: U256) {
        set_key(data::TOTAL_COLLECTED, total_collected);
    }

    fn get_total_collected(&self) -> U256 {
        get_key(data::TOTAL_COLLECTED).unwrap_or_revert()
    }

    //Balance contributors can claim at a given moment
    fn set_claimable_balance(&self, claimable_balance: U256) {
        set_key(data::CLAIMABLE_BALANCE, claimable_balance);
    }

    fn get_claimable_balance(&self) -> U256 {
        get_key(data::CLAIMABLE_BALANCE).unwrap_or_revert()
    }

    //Balance the locker owner still owes
    fn set_remaining_balance(&self, remaining_balance: U256) {
        set_key(data::REMAINING_BALANCE, remaining_balance);
    }

    fn get_remaining_balance(&self) -> U256 {
        get_key(data::REMAINING_BALANCE).unwrap_or_revert()
    }

    //Balance of all penalties incurred by locker owner so far
    fn set_penalties_balance(&self, penalties_balance: U256) {
        set_key(data::PENALTIES_BALANCE, penalties_balance);
    }

    fn get_penalties_balance(&self) -> U256 {
        get_key(data::PENALTIES_BALANCE).unwrap_or_revert()
    }

    //Time next payoff must happen to avoid penalties
    fn set_next_due_time(&self, next_due_time: U256) {
        set_key(data::NEXT_DUE_TIME, next_due_time);
    }

    fn get_next_due_time(&self) -> U256 {
        get_key(data::NEXT_DUE_TIME).unwrap_or_revert()
    }

    //Timestamp initialize was called
    fn set_creation_time(&self, creation_time: U256) {
        set_key(data::CREATION_TIME, creation_time);
    }

    fn get_creation_time(&self) -> U256 {
        get_key(data::CREATION_TIME).unwrap_or_revert()
    }
}
