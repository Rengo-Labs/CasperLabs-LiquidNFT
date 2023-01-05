use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ContractPackageHash, Key};
use casperlabs_contract_utils::{get_key, set_key};

pub use common::keys::*;

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
