use alloc::vec::Vec;
use casper_types::{Key, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casperlabs_contract_utils::{get_key, set_key, Dict};

pub use common::keys::*;

pub const FEE: U256 = U256([20, 0, 0, 0]);
pub const DEADLINE_TIME: U256 = U256([604800, 0, 0, 0]);
pub const CONTRIBUTION_TIME: U256 = U256([432000, 0, 0, 0]);
pub const SECONDS_IN_DAY: U256 = U256([86400, 0, 0, 0]);

#[derive(Debug, Clone, CLTyped, ToBytes, FromBytes)]
pub struct Globals {
    pub token_id: Vec<U256>,
    pub payment_time: U256,
    pub payment_rate: U256,
    pub locker_owner: Key,
    pub token_address: Key,
}
impl Default for Globals {
    fn default() -> Self {
        Globals {
            token_id: Vec::new(),
            payment_time: 0.into(),
            payment_rate: 0.into(),
            locker_owner: zero_address(),
            token_address: zero_address(),
        }
    }
}

pub fn set_globals(globals: Globals) {
    set_key(GLOBALS, globals);
}

pub fn get_globals() -> Globals {
    get_key(GLOBALS).unwrap_or_default()
}

pub struct Contributions {
    dict: Dict,
}

impl Contributions {
    pub fn instance() -> Contributions {
        Contributions {
            dict: Dict::instance(CONTRIBUTIONS_DICT),
        }
    }

    pub fn init() {
        Dict::init(CONTRIBUTIONS_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub struct Compensations {
    dict: Dict,
}

impl Compensations {
    pub fn instance() -> Compensations {
        Compensations {
            dict: Dict::instance(COMPENSATIONS_DICT),
        }
    }

    pub fn init() {
        Dict::init(COMPENSATIONS_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub fn set_payment_token(payment_token: Key) {
    set_key(PAYMENT_TOKEN, payment_token);
}

pub fn get_payment_token() -> Key {
    get_key(PAYMENT_TOKEN).unwrap_or_else(zero_address)
}

pub fn set_trustee_multisig(trustee_multisig: Key) {
    set_key(TRUSTEE_MULTISIG, trustee_multisig);
}

pub fn get_trustee_multisig() -> Key {
    get_key(TRUSTEE_MULTISIG).unwrap_or_else(zero_address)
}
