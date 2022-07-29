#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLValue, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef, U256,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};

use liquid_locker_crate::{
    self,
    entry_points::get_entry_points,
    liquid_helper_crate::{
        liquid_base_crate::{data::get_payment_token, LIQUIDBASE},
        LIQUIDHELPER,
    },
    LIQUIDLOCKER, LIQUIDTRANSFER,
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
impl LIQUIDTRANSFER<OnChainContractStorage> for LiquidLocker {}

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

/// @dev This is a call made by the constructor to set up variables on a new locker.
/// This is essentially equivalent to a constructor, but for our gas saving cloning operation instead.
/// This may also be used in locker-reuse in version 2.
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

/// @dev If the owner has missed payments by 7 days this call will transfer the NFT to either the
/// singleProvider address or the trusted multisig to be auctioned
#[no_mangle]
fn liquidate_locker() {
    LiquidLocker::default().liquidate_locker();
}

/// @dev Claim payed back tokens as a single contributor
#[no_mangle]
fn claim_interest_single() {
    LiquidLocker::default().claim_interest_single();
}

/// @dev Claim payed back tokens as with multiple contributors.
/// We need 2 functions because we cannot wipe all the contributions of users before someone became the sole contributor
#[no_mangle]
fn claim_interest_public() {
    LiquidLocker::default().claim_interest_public();
}

/// @dev During the contribution phase, the owner can decrease the duration of the loan.
/// The owner can only decrease the loan to a shorter duration, he cannot make it longer once the
/// contribution phase has started.
#[no_mangle]
fn decrease_payment_time() {
    let new_payment_rate: U256 = runtime::get_named_arg("new_payment_rate");
    LiquidLocker::default().decrease_payment_time(new_payment_rate);
}

/// @dev During the contribution phase, the owner can increase the rate they will pay for the loan.
/// The owner can only increase the rate to make the deal better for contributors, he cannot decrease it.
#[no_mangle]
fn increase_payment_rate() {
    let new_payment_rate: U256 = runtime::get_named_arg("new_payment_rate");
    LiquidLocker::default().increase_payment_rate(new_payment_rate);
}

/// @dev Locker owner calls this once the contribution phase is over to receive the funds for the loan.
/// This can only be done once the floor is reached, and can be done before the end of the contribution phase
/// if the floor is reached early. The owner can also prepay an amount to pay off some of the earnings at enable time.
/// The locker owner owes the earnings linearly until the end, then all of the actual loan plus any penalties are due at the end.
#[no_mangle]
fn enable_locker() {
    let prepay_amount: U256 = runtime::get_named_arg("prepay_amount");
    LiquidLocker::default().enable_locker(prepay_amount);
}

/// @dev If the floor asked was not reached during contributions, this function will return the nft to the owner
/// and allow all the contributors to claim their funds back.
#[no_mangle]
fn disable_locker() {
    LiquidLocker::default().disable_locker();
}

/// @dev There are a couple edge cases with extreme payment rates that cause enableLocker to revert.
/// These are never callable on our UI and doing so would require a manual transaction.
/// This function will disable a locker in this senario, allow contributors to claim their money and transfer the NFT back to the owner.
/// Only the team multisig has permission to do this
#[no_mangle]
fn rescue_locker() {
    LiquidLocker::default().rescue_locker();
}

/// @dev Allow users to claim funds when a locker is disabled
#[no_mangle]
fn refund_due_disabled() {
    let refund_address: Key = runtime::get_named_arg("refund_address");
    LiquidLocker::default().refund_due_disabled(refund_address);
}

/// @dev Allow users to claim funds when a someone kicks them out to become the single provider
#[no_mangle]
fn refund_due_single() {
    let refund_address: Key = runtime::get_named_arg("refund_address");
    LiquidLocker::default().refund_due_single(refund_address);
}

/// @dev Someone can add funds to the locker and they will be split among the contributors
/// This does not count as a payment on the loan.
#[no_mangle]
fn donate_funds() {
    let donation_amount: U256 = runtime::get_named_arg("donation_amount");
    LiquidLocker::default().donate_funds(donation_amount);
}

/// @dev Locker owner can payback funds.
/// Penalties are given if the owner does not pay the earnings linearally over the loan duration.
/// If the owner pays back the earnings, loan amount, and penalties aka fully pays off the loan
/// they will be transfered their nft back
#[no_mangle]
fn pay_back_funds() {
    let payment_amount: U256 = runtime::get_named_arg("payment_amount");
    LiquidLocker::default().pay_back_funds(payment_amount);
}

/// @dev Calculate how many sends should be added before the next payoff is due based on payment amount
#[no_mangle]
fn calculate_epoch() {
    let total_value: U256 = runtime::get_named_arg("total_value");
    let payment_time: U256 = runtime::get_named_arg("payment_time");
    let payment_rate: U256 = runtime::get_named_arg("payment_rate");
    let ret: U256 =
        LiquidLocker::default().calculate_epoch(total_value, payment_time, payment_rate);
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
    let ret: (U256, U256, U256) =
        LiquidLocker::default().calculate_paybacks(total_value, payment_time, payment_rate);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @dev Helper for the days math of calcualte penalties.
/// Returns +1 per day before the 4th day and +2 for each day after the 4th day
#[no_mangle]
fn get_late_days() {
    let ret: U256 = LiquidLocker::default().get_late_days();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// @dev Public pure accessor for _getPenaltyAmount
#[no_mangle]
fn penalty_amount() {
    let total_collected: U256 = runtime::get_named_arg("total_collected");
    let late_days_amount: U256 = runtime::get_named_arg("late_days_amount");
    let ret: U256 = LiquidLocker::default().penalty_amount(total_collected, late_days_amount);
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
    let ret: (U256, U256) = LiquidLocker::default().make_contribution(token_amount, token_holder);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

// Variables
#[no_mangle]
fn payment_token() {
    runtime::ret(CLValue::from_t(get_payment_token()).unwrap_or_revert());
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
