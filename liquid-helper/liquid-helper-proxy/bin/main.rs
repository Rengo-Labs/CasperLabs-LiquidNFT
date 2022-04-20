#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group, Key,
    Parameter, RuntimeArgs, URef, U256,
};
pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let liquid_helper: Key = runtime::get_named_arg("liquid_helper");

    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::liquid_helper_key(),
        ContractPackageHash::from(liquid_helper.into_hash().unwrap_or_default()),
    );
}

#[no_mangle]
fn get_tokens() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let ret: Vec<U256> = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "get_tokens",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn ownerless_locker() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let ret: bool = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "ownerless_locker",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn floor_not_reached() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let ret: bool = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "floor_not_reached",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn not_single_provider() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let check_address: Key = runtime::get_named_arg("check_address");
    let ret: bool = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "not_single_provider",
        runtime_args! {
            "check_address" => check_address
        },
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn reached_total() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let contributor: Key = runtime::get_named_arg("contributor");
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    let ret: bool = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "reached_total",
        runtime_args! {
            "contributor" => contributor,
            "token_amount" => token_amount
        },
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn missed_activate() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let ret: bool = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "missed_activate",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn missed_deadline() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let ret: bool = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "missed_deadline",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn payment_time_not_set() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let ret: bool = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "payment_time_not_set",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn below_floor_asked() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let ret: bool = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "below_floor_asked",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn contribution_phase() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let ret: bool = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "contribution_phase",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn payback_timestamp() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let ret: U256 = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "payback_timestamp",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn starting_timestamp() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let ret: U256 = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "starting_timestamp",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn liquidate_to() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let ret: Key = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "liquidate_to",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}

#[no_mangle]
fn time_since() {
    let liquid_helper_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_helper_key());
    let time_stamp: U256 = runtime::get_named_arg("time_stamp");
    let ret: U256 = runtime::call_versioned_contract(
        liquid_helper_address,
        None,
        "time_since",
        runtime_args! {
            "time_stamp" => time_stamp
        },
    );
    mappings::set_key(&mappings::result_key(), ret);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("liquid_helper", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_tokens",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "ownerless_locker",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "floor_not_reached",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "not_single_provider",
        vec![Parameter::new("check_address", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reached_total",
        vec![
            Parameter::new("contributor", Key::cl_type()),
            Parameter::new("token_amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "missed_activate",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "missed_deadline",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "below_floor_asked",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "payment_time_not_set",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "contribution_phase",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "payback_timestamp",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "starting_timestamp",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "liquidate_to",
        vec![Parameter::new("time_stamp", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "time_since",
        vec![Parameter::new("time_stamp", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let liquid_helper: Key = runtime::get_named_arg("liquid_helper");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "liquid_helper" => liquid_helper
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
