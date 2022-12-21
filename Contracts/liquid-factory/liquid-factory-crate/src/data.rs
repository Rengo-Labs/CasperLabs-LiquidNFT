use casper_contract::contract_api::{runtime, storage};
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractPackageHash, Key, U256};
use casperlabs_contract_utils::{get_key, set_key};
pub use common::keys::*;
use core::convert::TryInto;

// Precision factor for interest rate 100E9
pub const RATE_MAX: U256 = U256([100_000_000_000, 0, 0, 0]);

pub fn set_result<T: ToBytes + CLTyped>(value: T) {
    match runtime::get_key(RESULT) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_default();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(RESULT, key);
        }
    }
}

pub fn set_default_token(default_token: Key) {
    set_key(DEFAULT_TOKEN, default_token);
}

pub fn get_default_token() -> Key {
    get_key(DEFAULT_TOKEN).unwrap_or_else(zero_address)
}

pub fn set_master_address(master_address: Key) {
    set_key(MASTER_ADDRESS, master_address);
}

pub fn get_master_address() -> Key {
    get_key(MASTER_ADDRESS).unwrap_or_else(zero_address)
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_else(zero_address)
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_PACKAGE_HASH, package_hash);
}

pub fn get_contract_package_hash() -> ContractPackageHash {
    get_key(SELF_PACKAGE_HASH).unwrap_or_default()
}
