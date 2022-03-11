#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use liquid_factory::{self, LIQUIDFACTORY};
use liquid_transfer_crate::LiquidTransfer;

#[derive(Default)]
struct LiquidFactory(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for LiquidFactory {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl LiquidTransfer<OnChainContractStorage> for LiquidFactory {}
impl LIQUIDFACTORY<OnChainContractStorage> for LiquidFactory {}

impl LiquidFactory {
    fn constructor(
        &mut self,
        default_count: U256,
        default_token: Key,
        default_target: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        LIQUIDFACTORY::init(
            self,
            default_count,
            default_token,
            default_target,
            Key::from(contract_hash),
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let default_count: U256 = runtime::get_named_arg("default_count");
    let default_token: Key = runtime::get_named_arg("default_token");
    let default_target: Key = runtime::get_named_arg("default_target");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");

    LiquidFactory::default().constructor(
        default_count,
        default_token,
        default_target,
        contract_hash,
        package_hash,
    );
}

#[no_mangle]
fn store_predictions() {
    let prediction_start: U256 = runtime::get_named_arg("prediction_start");
    let prediction_count: U256 = runtime::get_named_arg("prediction_count");
    let prediction_token: Key = runtime::get_named_arg("prediction_token");

    LiquidFactory::default().store_predictions(
        prediction_start,
        prediction_count,
        prediction_token,
    );
}

#[no_mangle]
fn predict_locker_address() {
    let index: U256 = runtime::get_named_arg("index");
    let factory: Key = runtime::get_named_arg("factory");
    let implementation: Key = runtime::get_named_arg("implementation");

    LiquidFactory::default().predict_locker_address(index, factory, implementation);
}

#[no_mangle]
fn update_default_target() {
    let new_default_target: Key = runtime::get_named_arg("new_default_target");

    LiquidFactory::default().update_default_target(new_default_target);
}

#[no_mangle]
fn update_implementation() {
    let token_address: Key = runtime::get_named_arg("token_address");
    let target_address: Key = runtime::get_named_arg("target_address");

    LiquidFactory::default().update_implementation(token_address, target_address);
}

#[no_mangle]
fn update_master() {
    let new_master: Key = runtime::get_named_arg("new_master");

    LiquidFactory::default().update_master(new_master);
}

#[no_mangle]
fn revoke_master() {
    LiquidFactory::default().revoke_master();
}

#[no_mangle]
fn create_liquid_locker() {
    let token_id: Vec<U256> = runtime::get_named_arg("token_id");
    let token_address: Key = runtime::get_named_arg("token_address");
    let floor_asked: U256 = runtime::get_named_arg("floor_asked");
    let total_asked: U256 = runtime::get_named_arg("total_asked");
    let payment_time: U256 = runtime::get_named_arg("payment_time");
    let payment_rate: U256 = runtime::get_named_arg("payment_rate");
    let payment_token: Key = runtime::get_named_arg("payment_token");

    LiquidFactory::default().create_liquid_locker(
        token_id,
        token_address,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
        payment_token,
    );
}

#[no_mangle]
fn create_empty_locker() {
    let payment_token: Key = runtime::get_named_arg("payment_token");

    let ret: Key = LiquidFactory::default().create_empty_locker(payment_token);

    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn contribute_to_locker() {
    let lockers_address: Key = runtime::get_named_arg("lockers_address");
    let payment_amount: U256 = runtime::get_named_arg("payment_amount");

    let ret: (U256, U256) =
        LiquidFactory::default().contribute_to_locker(lockers_address, payment_amount);

    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_implementation() {
    let payment_token: Key = runtime::get_named_arg("payment_token");

    let ret: Key = LiquidFactory::default().get_implementation(payment_token);

    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn donate_to_locker() {
    let lockers_address: Key = runtime::get_named_arg("lockers_address");
    let donation_amount: U256 = runtime::get_named_arg("donation_amount");

    LiquidFactory::default().donate_to_locker(lockers_address, donation_amount);
}

#[no_mangle]
fn payback_to_locker() {
    let lockers_address: Key = runtime::get_named_arg("lockers_address");
    let payment_amount: U256 = runtime::get_named_arg("payment_amount");

    LiquidFactory::default().payback_to_locker(lockers_address, payment_amount);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("default_count", U256::cl_type()),
            Parameter::new("default_token", Key::cl_type()),
            Parameter::new("default_target", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "store_predictions",
        vec![
            Parameter::new("prediction_start", U256::cl_type()),
            Parameter::new("prediction_count", U256::cl_type()),
            Parameter::new("prediction_token", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "predict_locker_address",
        vec![
            Parameter::new("index", U256::cl_type()),
            Parameter::new("factory", Key::cl_type()),
            Parameter::new("implementation", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update_default_target",
        vec![Parameter::new("new_default_target", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update_implementation",
        vec![
            Parameter::new("token_address", Key::cl_type()),
            Parameter::new("target_address", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update_master",
        vec![Parameter::new("new_master", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "revoke_master",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_liquid_locker",
        vec![
            Parameter::new("token_id", CLType::List(Box::new(U256::cl_type()))),
            Parameter::new("token_address", Key::cl_type()),
            Parameter::new("floor_asked", U256::cl_type()),
            Parameter::new("total_asked", U256::cl_type()),
            Parameter::new("payment_time", U256::cl_type()),
            Parameter::new("payment_rate", U256::cl_type()),
            Parameter::new("payment_token", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_empty_locker",
        vec![Parameter::new("payment_token", Key::cl_type())],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "contribute_to_locker",
        vec![
            Parameter::new("lockers_address", Key::cl_type()),
            Parameter::new("payment_amount", U256::cl_type()),
        ],
        CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_implementation",
        vec![Parameter::new("payment_token", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "donate_to_locker",
        vec![
            Parameter::new("lockers_address", Key::cl_type()),
            Parameter::new("donation_amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "payback_to_locker",
        vec![
            Parameter::new("lockers_address", Key::cl_type()),
            Parameter::new("payment_amount", U256::cl_type()),
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

    let default_count: U256 = runtime::get_named_arg("default_count");
    let default_token: Key = runtime::get_named_arg("default_token");
    let default_target: Key = runtime::get_named_arg("default_target");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "default_count" => default_count,
        "default_token" => default_token,
        "default_target" => default_target,
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
