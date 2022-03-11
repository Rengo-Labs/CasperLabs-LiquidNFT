use core::convert::TryInto;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractPackageHash, Key, U256};
use contract_utils::{get_key, set_key, Dict};

pub const MASTER_ADDRESS: &str = "master_address";
pub const LOCKER_COUNT: &str = "locker_count";
pub const DEFAULT_COUNT: &str = "default_count";
pub const DEFAULT_TOKEN: &str = "default_token";
pub const DEFAULT_TARGET: &str = "default_target";

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const RESULT: &str = "result";

pub const IMPLEMENTATIONS: &str = "implementations";
pub struct Implementations {
    dict: Dict,
}

impl Implementations {
    pub fn instance() -> Implementations {
        Implementations {
            dict: Dict::instance(IMPLEMENTATIONS),
        }
    }

    pub fn init() {
        Dict::init(IMPLEMENTATIONS)
    }

    pub fn get(&self, owner: &Key) -> Key {
        self.dict.get_by_key(owner).unwrap_or_revert()
    }

    pub fn set(&self, owner: &Key, value: Key) {
        self.dict.set_by_key(owner, value);
    }
}

pub const LOCKERS: &str = "lockers";
pub struct Lockers {
    dict: Dict,
}

impl Lockers {
    pub fn instance() -> Lockers {
        Lockers {
            dict: Dict::instance(LOCKERS),
        }
    }

    pub fn init() {
        Dict::init(LOCKERS)
    }

    pub fn get(&self, owner: &Key) -> bool {
        self.dict.get_by_key(owner).unwrap_or_revert()
    }

    pub fn set(&self, owner: &Key, value: bool) {
        self.dict.set_by_key(owner, value);
    }
}

pub fn set_result<T: ToBytes + CLTyped>(value: T) {
    match runtime::get_key(RESULT) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(RESULT, key);
        }
    }
}

pub fn set_default_count(default_count: U256) {
    set_key(DEFAULT_COUNT, default_count);
}

pub fn get_default_count() -> U256 {
    get_key(DEFAULT_COUNT).unwrap_or_revert()
}

pub fn set_default_token(default_token: Key) {
    set_key(DEFAULT_TOKEN, default_token);
}

pub fn get_default_token() -> Key {
    get_key(DEFAULT_TOKEN).unwrap_or_revert()
}

pub fn set_default_target(default_target: Key) {
    set_key(DEFAULT_TARGET, default_target);
}

pub fn get_default_target() -> Key {
    get_key(DEFAULT_TARGET).unwrap_or_revert()
}

pub fn get_zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}

pub fn set_locker_count(locker_count: U256) {
    set_key(LOCKER_COUNT, locker_count);
}

pub fn get_locker_count() -> U256 {
    get_key(LOCKER_COUNT).unwrap_or_revert()
}

pub fn set_master_address(master_address: Key) {
    set_key(MASTER_ADDRESS, master_address);
}

pub fn get_master_address() -> Key {
    get_key(MASTER_ADDRESS).unwrap_or_revert()
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_PACKAGE_HASH, package_hash);
}

pub fn get_contract_package_hash() -> ContractPackageHash {
    get_key(SELF_PACKAGE_HASH).unwrap_or_revert()
}
