use alloc::string::ToString;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped, Key, U256,
};
use contract_utils::Dict;

pub const FEE: u128 = 20;
pub const DEADLINE_TIME: u128 = 7 * 86_400;
pub const CONTRIBUTION_TIME: u128 = 5 * 86_400;
pub const SECONDS_IN_DAY: u128 = 86400;

pub const SINGLE_PROVIDER: &str = "single_provider";
pub const FLOOR_ASKED: &str = "floor_asked";
pub const TOTAL_ASKED: &str = "total_asked";
pub const TOTAL_COLLECTED: &str = "total_collected";
pub const CLAIMABLE_BALANCE: &str = "claimable_balance";
pub const REMAINING_BALANCE: &str = "remaining_balance";
pub const PENALTIES_BALANCE: &str = "penalties_balance";
pub const NEXT_DUE_TIME: &str = "next_due_time";
pub const CREATION_TIME: &str = "creation_time";

pub const CONTRIBUTIONS_DICT: &str = "contributions";
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

pub const COMPENSATIONS_DICT: &str = "compensations";
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

pub const GLOBALS: &str = "globals";
pub struct Globals {
    dict: Dict,
}

impl Globals {
    pub fn instance() -> Globals {
        Globals {
            dict: Dict::instance(GLOBALS),
        }
    }

    pub fn init() {
        Dict::init(GLOBALS)
    }

    pub fn get<T: FromBytes + CLTyped>(&self, owner: &str) -> T {
        self.dict.get(owner).unwrap_or_revert()
    }

    pub fn set<T: ToBytes + CLTyped>(&self, owner: &str, value: T) {
        self.dict.set(owner.to_string().as_str(), value);
    }
}
