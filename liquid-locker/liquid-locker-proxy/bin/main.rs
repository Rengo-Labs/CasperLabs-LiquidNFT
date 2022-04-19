#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, vec};

use casper_contract::{
    contract_api::{runtime, storage,},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256};
pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let liquid_locker: Key = runtime::get_named_arg("liquid_locker");
    
    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::liquid_locker_key(),
        ContractPackageHash::from(liquid_locker.into_hash().unwrap_or_default()),
    );
}

#[no_mangle]
fn liquidate_locker(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
        let ret:() = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "liquidate_locker",
            runtime_args! {},
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn claim_interest_single(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
        let ret:() = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "claim_interest_single",
            runtime_args! {},
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn claim_interest_public(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
        let ret:() = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "claim_interest_public",
            runtime_args! {},
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn decrease_payment_time(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
        let new_payment_rate: U256 = runtime::get_named_arg("new_payment_rate");
        let ret:() = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "decrease_payment_time",
            runtime_args! {
                "new_payment_rate" => new_payment_rate
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn increase_payment_rate(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
        let new_payment_rate: U256 = runtime::get_named_arg("new_payment_rate");
        let ret:() = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "increase_payment_rate",
            runtime_args! {
                "new_payment_rate" => new_payment_rate
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn enable_locker(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
    let prepay_amount: U256 = runtime::get_named_arg("prepay_amount");
        let ret:() = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "enable_locker",
            runtime_args! {
                "prepay_amount" => prepay_amount
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn disable_locker(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
        let ret:() = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "disable_locker",
            runtime_args! {
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn rescue_locker(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
        let ret:() = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "rescue_locker",
            runtime_args! {
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn refund_due_disabled(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
    let refund_address: Key = runtime::get_named_arg("refund_address");
        let ret:() = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "refund_due_disabled",
            runtime_args! {
                "refund_address" => refund_address
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn refund_due_single(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
    let refund_address: Key = runtime::get_named_arg("refund_address");
        let ret:() = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "refund_due_single",
            runtime_args! {
                "refund_address" => refund_address
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn donate_funds(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
    let donation_amount: Key = runtime::get_named_arg("donation_amount");
        let ret:() = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "donate_funds",
            runtime_args! {
                "donation_amount" => donation_amount
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn pay_back_funds(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
    let payment_amount: Key = runtime::get_named_arg("payment_amount");
        let ret:() = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "pay_back_funds",
            runtime_args! {
                "payment_amount" => payment_amount
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn calculate_epoch(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
    let total_value: U256 = runtime::get_named_arg("total_value");
    let payment_time: U256 = runtime::get_named_arg("payment_time");
    let payment_rate: U256 = runtime::get_named_arg("payment_rate");
        let ret:U256 = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "calculate_epoch",
            runtime_args! {
                "total_value" => total_value,
                "payment_time" => payment_time,
                "payment_rate" => payment_rate,
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn calculate_paybacks(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
    let total_value: U256 = runtime::get_named_arg("total_value");
    let payment_time: U256 = runtime::get_named_arg("payment_time");
    let payment_rate: U256 = runtime::get_named_arg("payment_rate");
        let ret:(U256,U256,U256) = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "calculate_paybacks",
            runtime_args! {
                "total_value" => total_value,
                "payment_time" => payment_time,
                "payment_rate" => payment_rate,
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn get_late_days(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
        let ret:U256 = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "get_late_days",
            runtime_args! {
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn penalty_amount(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
        let total_collected: U256 = runtime::get_named_arg("total_collected");
        let late_days_amount: U256 = runtime::get_named_arg("late_days_amount");
        let ret:U256 = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "penalty_amount",
            runtime_args! {
                "total_collected" => total_collected,
                "late_days_amount" => late_days_amount
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn make_contribution(){
    let liquid_locker_address: ContractPackageHash =
        mappings::get_key(&mappings::liquid_locker_key());
        let token_amount:U256 = runtime::get_named_arg("token_amount");
        let token_holder: Key = runtime::get_named_arg("token_holder");
        let ret:(U256,U256) = runtime::call_versioned_contract(
            liquid_locker_address,
            None,
            "make_contribution",
            runtime_args! {
                "token_amount" => token_amount,
                "token_holder" => token_holder
            },
        );
       mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn set_liquid_locker(){
    let token: Key = runtime::get_named_arg("token");
    mappings::set_key(
        &mappings::liquid_locker_key(),
        ContractHash::from(token.into_hash().unwrap_or_revert()),
    );
}
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("liquid_locker", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "initialize",
        vec![
            Parameter::new("token_id", CLType::List(Box::new(CLType::U256))),
            Parameter::new("token_address", CLType::Key),
            Parameter::new("token_owner", CLType::Key),
            Parameter::new("floor_asked", CLType::U256),
            Parameter::new("total_asked", CLType::U256),
            Parameter::new("payment_time", CLType::U256),
            Parameter::new("payment_rate", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_payment_rate",
        vec![Parameter::new("new_payment_rate", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decrease_payment_time",
        vec![Parameter::new("new_payment_rate", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "enable_locker",
        vec![Parameter::new("prepay_amount", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "disable_locker",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "rescue_locker",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "refund_due_disabled",
        vec![Parameter::new("refund_address", CLType::Key)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "refund_due_single",
        vec![Parameter::new("refund_address", CLType::Key)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "donate_funds",
        vec![Parameter::new("donation_amount", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "pay_back_funds",
        vec![Parameter::new("payment_amount", CLType::U256)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "liquidate_locker",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claim_interest_single",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claim_interest_public",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "calculate_epoch",
        vec![
            Parameter::new("total_value", CLType::U256),
            Parameter::new("payment_time", CLType::U256),
            Parameter::new("payment_rate", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "calculate_paybacks",
        vec![
            Parameter::new("total_value", CLType::U256),
            Parameter::new("payment_time", CLType::U256),
            Parameter::new("payment_rate", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_late_days",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "penalty_amount",
        vec![
            Parameter::new("total_collected", CLType::U256),
            Parameter::new("late_days_amount", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "make_contribution",
        vec![
            Parameter::new("token_amount", CLType::U256),
            Parameter::new("token_holder", CLType::Key),
        ],
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
    let liquid_locker: Key = runtime::get_named_arg("liquid_locker");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "liquid_locker" => liquid_locker
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
