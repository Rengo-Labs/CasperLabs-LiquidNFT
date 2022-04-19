#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, string::String, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};

use liquid_transfer_crate::LIQUIDTRANSFER;

#[derive(Default)]
struct LiquidTransfer(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for LiquidTransfer {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl LIQUIDTRANSFER<OnChainContractStorage> for LiquidTransfer {}
impl LiquidTransfer {
    fn constructor(
        &mut self,
        punks: Key,
        kitties: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        LIQUIDTRANSFER::init(self, punks, kitties, Key::from(contract_hash), package_hash);
    }
}

#[no_mangle]
fn constructor() {
    let punks: Key = runtime::get_named_arg("punks");
    let kitties: Key = runtime::get_named_arg("kitties");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    LiquidTransfer::default().constructor(punks, kitties, contract_hash, package_hash);
}

#[no_mangle]
fn transfer_nft() {
    let from: Key = runtime::get_named_arg("punks");
    let to: Key = runtime::get_named_arg("punks");
    let token_address: Key = runtime::get_named_arg("punks");
    let token_id: U256 = runtime::get_named_arg("punks");

    LiquidTransfer::default().transfer_nft(from, to, token_address, token_id);
}

#[no_mangle]
fn transfer_from_nft() {
    let from: Key = runtime::get_named_arg("punks");
    let to: Key = runtime::get_named_arg("punks");
    let token_address: Key = runtime::get_named_arg("punks");
    let token_id: U256 = runtime::get_named_arg("punks");

    LiquidTransfer::default().transfer_from_nft(from, to, token_address, token_id);
}

#[no_mangle]
fn on_erc721_received() {
    let call_function: String = runtime::get_named_arg("call_function");
    let operator: Key = runtime::get_named_arg("operator");
    let from: Key = runtime::get_named_arg("from");
    let token_id: U256 = runtime::get_named_arg("token_id");
    let data: String = runtime::get_named_arg("data");

    let ret: String =
        LiquidTransfer::default().on_erc721_received(call_function, operator, from, token_id, data);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("punks", Key::cl_type()),
            Parameter::new("kitties", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_nft",
        vec![
            Parameter::new("from", CLType::Key),
            Parameter::new("to", CLType::Key),
            Parameter::new("token_address", CLType::Key),
            Parameter::new("token_id", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from_nft",
        vec![
            Parameter::new("from", CLType::Key),
            Parameter::new("to", CLType::Key),
            Parameter::new("token_address", CLType::Key),
            Parameter::new("token_id", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "on_erc721_received",
        vec![
            Parameter::new("call_function", CLType::String),
            Parameter::new("operator", CLType::Key),
            Parameter::new("from", CLType::Key),
            Parameter::new("token_id", CLType::U256),
            Parameter::new("data", CLType::String),
        ],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    let punks: Key = runtime::get_named_arg("punks");
    let kitties: Key = runtime::get_named_arg("kitties");

    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "punks" => punks,
        "kitties" => kitties,
        "contract_hash" => contract_hash,
        "package_hash"=> package_hash
    };

    // Add the constructor group to the package hash with a single URef.
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    // Call the constructor entry point
    let _: () =
        runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    // Store contract in the account's named keys.
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    runtime::put_key(
        &format!("{}_package_hash", contract_name),
        package_hash.into(),
    );
    runtime::put_key(
        &format!("{}_package_hash_wrapped", contract_name),
        storage::new_uref(package_hash).into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_name),
        storage::new_uref(contract_hash).into(),
    );
    runtime::put_key(
        &format!("{}_package_access_token", contract_name),
        access_token.into(),
    );
}
