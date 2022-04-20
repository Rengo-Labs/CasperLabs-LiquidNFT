#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, vec, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};

use liquid_locker_crate::{
    self,
    liquid_helper_crate::{liquid_base_crate::LIQUIDBASE, LIQUIDHELPER},
    LIQUIDLOCKER,
};

#[derive(Default)]
struct LiquidLocker(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for LiquidLocker {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl LIQUIDHELPER<OnChainContractStorage> for LiquidLocker {}
impl LIQUIDBASE<OnChainContractStorage> for LiquidLocker {}

impl LIQUIDLOCKER<OnChainContractStorage> for LiquidLocker {}
impl LiquidLocker {
    fn constructor(
        &mut self,
        trustee_multisig: Key,
        payment_token: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        LIQUIDLOCKER::init(
            self,
            trustee_multisig,
            payment_token,
            Key::from(contract_hash),
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let trustee_multisig: Key = runtime::get_named_arg("trustee_multisig");
    let payment_token: Key = runtime::get_named_arg("payment_token");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    LiquidLocker::default().constructor(
        trustee_multisig,
        payment_token,
        contract_hash,
        package_hash,
    );
}
#[no_mangle]
fn initialize() {
    let token_id: Vec<U256> = runtime::get_named_arg("token_id");
    let token_address: Key = runtime::get_named_arg("token_address");
    let token_owner: Key = runtime::get_named_arg("token_owner");
    let floor_asked: U256 = runtime::get_named_arg("floor_asked");
    let total_asked: U256 = runtime::get_named_arg("total_asked");
    let payment_time: U256 = runtime::get_named_arg("payment_time");
    let payment_rate: U256 = runtime::get_named_arg("payment_rate");
    LiquidLocker::default().initialize(
        token_id,
        token_address,
        token_owner,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
    );
}
#[no_mangle]
fn liquidate_locker() {
    LiquidLocker::default().liquidate_locker();
}
#[no_mangle]
fn claim_interest_single() {
    LiquidLocker::default().claim_interest_single();
}
#[no_mangle]
fn claim_interest_public() {
    LiquidLocker::default().claim_interest_public();
}
#[no_mangle]
fn decrease_payment_time() {
    let new_payment_rate: U256 = runtime::get_named_arg("new_payment_rate");
    LiquidLocker::default().decrease_payment_time(new_payment_rate);
}
#[no_mangle]
fn increase_payment_rate() {
    let new_payment_rate: U256 = runtime::get_named_arg("new_payment_rate");
    LiquidLocker::default().increase_payment_rate(new_payment_rate);
}
#[no_mangle]
fn enable_locker() {
    let prepay_amount: U256 = runtime::get_named_arg("prepay_amount");
    LiquidLocker::default().enable_locker(prepay_amount);
}
#[no_mangle]
fn disable_locker() {
    LiquidLocker::default().disable_locker();
}
#[no_mangle]
fn rescue_locker() {
    LiquidLocker::default().rescue_locker();
}
#[no_mangle]
fn refund_due_disabled() {
    let refund_address: Key = runtime::get_named_arg("refund_address");
    LiquidLocker::default().refund_due_disabled(refund_address);
}

#[no_mangle]
fn refund_due_single() {
    let refund_address: Key = runtime::get_named_arg("refund_address");
    LiquidLocker::default().refund_due_single(refund_address);
}
#[no_mangle]
fn donate_funds() {
    let donation_amount: U256 = runtime::get_named_arg("donation_amount");
    LiquidLocker::default().donate_funds(donation_amount);
}
#[no_mangle]
fn pay_back_funds() {
    let payment_amount: U256 = runtime::get_named_arg("payment_amount");
    LiquidLocker::default().pay_back_funds(payment_amount);
}
#[no_mangle]
fn calculate_epoch() {
    let total_value: U256 = runtime::get_named_arg("total_value");
    let payment_time: U256 = runtime::get_named_arg("payment_time");
    let payment_rate: U256 = runtime::get_named_arg("payment_rate");
    let ret: U256 =
        LiquidLocker::default().calculate_epoch(total_value, payment_time, payment_rate);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn calculate_paybacks() {
    let total_value: U256 = runtime::get_named_arg("total_value");
    let payment_time: U256 = runtime::get_named_arg("payment_time");
    let payment_rate: U256 = runtime::get_named_arg("payment_rate");
    let ret: (U256, U256, U256) =
        LiquidLocker::default().calculate_paybacks(total_value, payment_time, payment_rate);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn get_late_days() {
    let ret: U256 = LiquidLocker::default().get_late_days();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn penalty_amount() {
    let total_collected: U256 = runtime::get_named_arg("total_collected");
    let late_days_amount: U256 = runtime::get_named_arg("late_days_amount");
    let ret: U256 = LiquidLocker::default().penalty_amount(total_collected, late_days_amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn make_contribution() {
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    let token_holder: Key = runtime::get_named_arg("token_holder");
    let ret: (U256, U256) = LiquidLocker::default().make_contribution(token_amount, token_holder);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("trustee_multisig", Key::cl_type()),
            Parameter::new("payment_token", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
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
        U256::cl_type(),
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
        CLType::Tuple3([
            Box::new(CLType::U256),
            Box::new(CLType::U256),
            Box::new(CLType::U256),
        ]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_late_days",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "penalty_amount",
        vec![
            Parameter::new("total_collected", CLType::U256),
            Parameter::new("late_days_amount", CLType::U256),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "make_contribution",
        vec![
            Parameter::new("token_amount", CLType::U256),
            Parameter::new("token_holder", CLType::Key),
        ],
        CLType::Tuple2([Box::new(CLType::U256), Box::new(CLType::U256)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Store contract in the account's named keys. Contract name must be same for all new versions of the contracts
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        // add a first version to this package
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        let trustee_multisig: Key = runtime::get_named_arg("trustee_multisig");
        let payment_token: Key = runtime::get_named_arg("payment_token");
        let constructor_args = runtime_args! {
            "trustee_multisig" => trustee_multisig,
            "payment_token" => payment_token,
            "package_hash" => package_hash,
            "contract_hash" => contract_hash,
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
    // If contract package did already exist
    else {
        // get the package
        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();
        // create new version and install it
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}