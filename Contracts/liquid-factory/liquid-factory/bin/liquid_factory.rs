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
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use liquid_factory_crate::{
    self,
    data::set_result,
    liquid_locker_crate::{
        entry_points,
        liquid_helper_crate::{
            liquid_base_crate::{data::get_payment_token, LIQUIDBASE},
            LIQUIDHELPER,
        },
        LIQUIDLOCKER,
    },
    liquid_transfer_crate::LIQUIDTRANSFER,
    LIQUIDFACTORY,
};

#[derive(Default)]
struct LiquidFactory(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for LiquidFactory {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl LIQUIDTRANSFER<OnChainContractStorage> for LiquidFactory {}
impl LIQUIDBASE<OnChainContractStorage> for LiquidFactory {}
impl LIQUIDHELPER<OnChainContractStorage> for LiquidFactory {}
impl LIQUIDLOCKER<OnChainContractStorage> for LiquidFactory {}
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
fn factory_constructor() {
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
/// Transfer master permission
#[no_mangle]
fn update_master() {
    let new_master: Key = runtime::get_named_arg("new_master");

    LiquidFactory::default().update_master(new_master);
}
/// Destroy Master functionality
#[no_mangle]
fn revoke_master() {
    LiquidFactory::default().revoke_master();
}
/// @dev Clone the implemenation for a token into a new contract.
/// Call into initialize for the locker to begin the LiquidNFT loan process.
/// Transfer the NFT the user wants use for the loan into the locker.
#[no_mangle]
fn create_liquid_locker() {
    let token_id: Vec<U256> = runtime::get_named_arg("token_id");
    let token_address: Key = runtime::get_named_arg("token_address");
    let floor_asked: U256 = runtime::get_named_arg("floor_asked");
    let total_asked: U256 = runtime::get_named_arg("total_asked");
    let payment_time: U256 = runtime::get_named_arg("payment_time");
    let payment_rate: U256 = runtime::get_named_arg("payment_rate");
    let payment_token: Key = runtime::get_named_arg("payment_token");

    let ret = LiquidFactory::default().create_liquid_locker(
        token_id,
        token_address,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
        payment_token,
    );
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn create_liquid_locker_js_client() {
    let token_id: Vec<U256> = runtime::get_named_arg("token_id");
    let token_address: Key = runtime::get_named_arg("token_address");
    let floor_asked: U256 = runtime::get_named_arg("floor_asked");
    let total_asked: U256 = runtime::get_named_arg("total_asked");
    let payment_time: U256 = runtime::get_named_arg("payment_time");
    let payment_rate: U256 = runtime::get_named_arg("payment_rate");
    let payment_token: Key = runtime::get_named_arg("payment_token");

    let ret: (Key, Key) = LiquidFactory::default().create_liquid_locker(
        token_id,
        token_address,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
        payment_token,
    );
    set_result(ret);
}

#[no_mangle]
fn create_empty_locker() {
    let payment_token: Key = runtime::get_named_arg("payment_token");

    let ret: (Key, Key) = LiquidFactory::default().create_empty_locker(payment_token);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn create_empty_locker_js_client() {
    let payment_token: Key = runtime::get_named_arg("payment_token");

    let ret: (Key, Key) = LiquidFactory::default().create_empty_locker(payment_token);
    set_result(ret);
}
/// @dev Call contributeToLocker. Factory acts as a middle man between the user and the locker.
/// We do this so that the user only has to approve the factory and not each new locker.
#[no_mangle]
fn contribute_to_locker() {
    let lockers_address: Key = runtime::get_named_arg("lockers_address");
    let payment_amount: U256 = runtime::get_named_arg("payment_amount");

    let ret: (U256, U256) =
        LiquidFactory::default().contribute_to_locker(lockers_address, payment_amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn contribute_to_locker_js_client() {
    let lockers_address: Key = runtime::get_named_arg("lockers_address");
    let payment_amount: U256 = runtime::get_named_arg("payment_amount");

    let ret: (U256, U256) =
        LiquidFactory::default().contribute_to_locker(lockers_address, payment_amount);
    set_result(ret);
}
/// dev Give tokens to a locker. These tokens do not go toward paying off the loan,
/// they are instead distributed among the contributors for the loan.
/// The result of this is that the value is transferred to the contributors not the owner because it does
/// not deduct from the balance the owner owes.
#[no_mangle]
fn donate_to_locker() {
    let lockers_address: Key = runtime::get_named_arg("lockers_address");
    let donation_amount: U256 = runtime::get_named_arg("donation_amount");

    LiquidFactory::default().donate_to_locker(lockers_address, donation_amount);
}
/// @dev Call paybackToLocker. Factory acts as a middle man between the user and the locker.
/// We do this so that the user only has to approve the factory and not each new locker.
#[no_mangle]
fn payback_to_locker() {
    let lockers_address: Key = runtime::get_named_arg("lockers_address");
    let payment_amount: U256 = runtime::get_named_arg("payment_amount");

    LiquidFactory::default().payback_to_locker(lockers_address, payment_amount);
}

// Liquid Locker 'no_mangle' functions

#[no_mangle]
fn constructor() {
    let trustee_multisig: Key = runtime::get_named_arg("trustee_multisig");
    let payment_token: Key = runtime::get_named_arg("payment_token");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    LIQUIDLOCKER::init(
        &mut LiquidFactory::default(),
        trustee_multisig,
        payment_token,
        Key::from(contract_hash),
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
    LIQUIDLOCKER::initialize(
        &LiquidFactory::default(),
        token_id,
        token_address,
        token_owner,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
    );
}
/// @dev If the owner has missed payments by 7 days this call will transfer the NFT to either the
/// singleProvider address or the trusted multisig to be auctioned
#[no_mangle]
fn liquidate_locker() {
    LIQUIDLOCKER::liquidate_locker(&LiquidFactory::default());
}
/// @dev Claim payed back tokens as a single contributor
#[no_mangle]
fn claim_interest_single() {
    LIQUIDLOCKER::claim_interest_single(&LiquidFactory::default());
}
/// @dev Claim payed back tokens as with multiple contributors.
/// We need 2 functions because we cannot wipe all the contributions of users before someone became the sole contributor
#[no_mangle]
fn claim_interest_public() {
    LIQUIDLOCKER::claim_interest_public(&LiquidFactory::default());
}
/// @dev During the contribution phase, the owner can decrease the duration of the loan.
/// The owner can only decrease the loan to a shorter duration, he cannot make it longer once the
/// contribution phase has started.
#[no_mangle]
fn decrease_payment_time() {
    let new_payment_rate: U256 = runtime::get_named_arg("new_payment_rate");
    LIQUIDLOCKER::decrease_payment_time(&LiquidFactory::default(), new_payment_rate);
}
/// @dev During the contribution phase, the owner can increase the rate they will pay for the loan.
/// The owner can only increase the rate to make the deal better for contributors, he cannot decrease it.
#[no_mangle]
fn increase_payment_rate() {
    let new_payment_rate: U256 = runtime::get_named_arg("new_payment_rate");
    LIQUIDLOCKER::increase_payment_rate(&LiquidFactory::default(), new_payment_rate);
}
/// @dev Locker owner calls this once the contribution phase is over to receive the funds for the loan.
/// This can only be done once the floor is reached, and can be done before the end of the contribution phase
/// if the floor is reached early. The owner can also prepay an amount to pay off some of the earnings at enable time.
/// The locker owner owes the earnings linearly until the end, then all of the actual loan plus any penalties are due at the end.
#[no_mangle]
fn enable_locker() {
    let prepay_amount: U256 = runtime::get_named_arg("prepay_amount");
    LIQUIDLOCKER::enable_locker(&mut LiquidFactory::default(), prepay_amount);
}
/// @dev If the floor asked was not reached during contributions, this function will return the nft to the owner
/// and allow all the contributors to claim their funds back.
#[no_mangle]
fn disable_locker() {
    LIQUIDLOCKER::disable_locker(&LiquidFactory::default());
}
/// @dev There are a couple edge cases with extreme payment rates that cause enableLocker to revert.
/// These are never callable on our UI and doing so would require a manual transaction.
/// This function will disable a locker in this senario, allow contributors to claim their money and transfer the NFT back to the owner.
/// Only the team multisig has permission to do this
#[no_mangle]
fn rescue_locker() {
    LIQUIDLOCKER::rescue_locker(&LiquidFactory::default());
}
/// @dev Allow users to claim funds when a locker is disabled
#[no_mangle]
fn refund_due_disabled() {
    let refund_address: Key = runtime::get_named_arg("refund_address");
    LIQUIDLOCKER::refund_due_disabled(&LiquidFactory::default(), refund_address);
}
/// @dev Allow users to claim funds when a someone kicks them out to become the single provider
#[no_mangle]
fn refund_due_single() {
    let refund_address: Key = runtime::get_named_arg("refund_address");
    LIQUIDLOCKER::refund_due_single(&LiquidFactory::default(), refund_address);
}
/// @dev Someone can add funds to the locker and they will be split among the contributors
/// This does not count as a payment on the loan.
#[no_mangle]
fn donate_funds() {
    let donation_amount: U256 = runtime::get_named_arg("donation_amount");
    LIQUIDLOCKER::donate_funds(&LiquidFactory::default(), donation_amount);
}
/// @dev Locker owner can payback funds.
/// Penalties are given if the owner does not pay the earnings linearally over the loan duration.
/// If the owner pays back the earnings, loan amount, and penalties aka fully pays off the loan
/// they will be transfered their nft back
#[no_mangle]
fn pay_back_funds() {
    let payment_amount: U256 = runtime::get_named_arg("payment_amount");
    LIQUIDLOCKER::pay_back_funds(&mut LiquidFactory::default(), payment_amount);
}
/// @dev Calculate how many sends should be added before the next payoff is due based on payment amount
#[no_mangle]
fn calculate_epoch() {
    let total_value: U256 = runtime::get_named_arg("total_value");
    let payment_time: U256 = runtime::get_named_arg("payment_time");
    let payment_rate: U256 = runtime::get_named_arg("payment_rate");
    let ret: U256 = LIQUIDLOCKER::calculate_epoch(
        &LiquidFactory::default(),
        total_value,
        payment_time,
        payment_rate,
    );
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
/// @dev Calulate how much the usage fee takes off a payments,
/// and how many tokens are due per second of loan
/// (epochPayback is amount of tokens to extend loan by 1 second. Only need to pay off earnings)
#[no_mangle]
fn calculate_paybacks() {
    let total_value: U256 = runtime::get_named_arg("total_value");
    let payment_time: U256 = runtime::get_named_arg("payment_time");
    let payment_rate: U256 = runtime::get_named_arg("payment_rate");
    let ret: (U256, U256, U256) = LIQUIDLOCKER::calculate_paybacks(
        &LiquidFactory::default(),
        total_value,
        payment_time,
        payment_rate,
    );
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
/// @dev Helper for the days math of calcualte penalties.
/// Returns +1 per day before the 4th day and +2 for each day after the 4th day
#[no_mangle]
fn get_late_days() {
    let ret: U256 = LIQUIDLOCKER::get_late_days(&LiquidFactory::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
/// @dev Public pure accessor for _getPenaltyAmount
#[no_mangle]
fn penalty_amount() {
    let total_collected: U256 = runtime::get_named_arg("total_collected");
    let late_days_amount: U256 = runtime::get_named_arg("late_days_amount");
    let ret: U256 =
        LIQUIDLOCKER::penalty_amount(&LiquidFactory::default(), total_collected, late_days_amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
/// @dev Public users can add tokens to the pool to be used for the loan.
/// The contributions for each user along with the total are recorded for splitting funds later.
/// If a user contributes up to the maximum asked on a loan, they will become the sole provider
/// (See _usersIncrease and _reachedTotal for functionality on becoming the sole provider)
/// The sole provider will receive the token instead of the trusted multisig in the case if a liquidation.
#[no_mangle]
fn make_contribution() {
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    let token_holder: Key = runtime::get_named_arg("token_holder");
    let ret: (U256, U256) =
        LIQUIDLOCKER::make_contribution(&mut LiquidFactory::default(), token_amount, token_holder);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
// Variables
#[no_mangle]
fn payment_token() {
    runtime::ret(CLValue::from_t(get_payment_token()).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "factory_constructor",
        vec![
            Parameter::new("default_count", U256::cl_type()),
            Parameter::new("default_token", Key::cl_type()),
            Parameter::new("default_target", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("factory_constructor")]),
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
        CLType::Tuple2([Box::new(CLType::Key), Box::new(CLType::Key)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_liquid_locker_js_client",
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
        CLType::Tuple2([Box::new(CLType::Key), Box::new(CLType::Key)]),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_empty_locker_js_client",
        vec![Parameter::new("payment_token", Key::cl_type())],
        <()>::cl_type(),
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
        "contribute_to_locker_js_client",
        vec![
            Parameter::new("lockers_address", Key::cl_type()),
            Parameter::new("payment_amount", U256::cl_type()),
        ],
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
    // Adding Liquid Locker contract entry points in Liquid Factory contract during iterations
    let liquid_locker_entry_points = entry_points::get_entry_points().take_entry_points();
    for entry_point in liquid_locker_entry_points {
        entry_points.add_entry_point(entry_point);
    }
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
    let constructor_access: URef = storage::create_contract_user_group(
        package_hash,
        "factory_constructor",
        1,
        Default::default(),
    )
    .unwrap_or_revert()
    .pop()
    .unwrap_or_revert();

    // Call the constructor entry point
    let _: () = runtime::call_versioned_contract(
        package_hash,
        None,
        "factory_constructor",
        constructor_args,
    );

    // Remove all URefs from the constructor group, so no one can call it for the second time.
    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "factory_constructor", urefs)
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
