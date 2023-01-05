use alloc::vec::Vec;
use casper_contract::{
    contract_api::runtime::{self, get_blocktime},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use liquid_base_crate::{self, data::*, LIQUIDBASE};

use common::errors::*;

pub trait LIQUIDHELPER<Storage: ContractStorage>:
    ContractContext<Storage> + LIQUIDBASE<Storage>
{
    fn init(
        &mut self,
        factory_address: Key,
        trustee_multisig: Key,
        payment_token: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        LIQUIDBASE::init(
            self,
            factory_address,
            trustee_multisig,
            payment_token,
            contract_hash,
            package_hash,
        );
    }

    /// @dev returns IDs of NFTs being held
    fn get_tokens(&self) -> Vec<U256> {
        get_globals().token_id
    }

    /// @dev returns true if contributions have not reached min asked
    fn floor_not_reached(&self) -> bool {
        !self.contribution_phase() && self.below_floor_asked()
    }

    /// @dev returns true if the provider address is not the single provider
    fn not_single_provider(&self, check_address: Key) -> bool {
        get_single_provider() != check_address
            && get_single_provider() != zero_address()
            && get_single_provider() != account_zero_address()
    }

    /// @dev returns true if the contributor will reach the ceiling asked with the provided token amount
    fn reached_total(&self, contributor: Key, token_amount: U256) -> bool {
        Contributions::instance()
            .get(&contributor)
            .checked_add(token_amount)
            .unwrap_or_revert()
            >= get_total_asked()
    }

    /// @dev returns true if locker has not been enabled within 7 days after contribution phase
    fn missed_activate(&self) -> bool {
        self.floor_not_reached()
            && self
                .starting_timestamp()
                .checked_add(DEADLINE_TIME)
                .unwrap_or_revert()
                < U256::from(u64::from(get_blocktime()))
    }

    /// @dev returns true if owner has not paid back within 7 days of last payment
    fn missed_deadline(&self) -> bool {
        let next_due_or_deadline = if get_next_due_time() > self.payback_timestamp() {
            self.payback_timestamp()
        } else {
            get_next_due_time()
        };
        get_next_due_time() > 0.into()
            && next_due_or_deadline
                .checked_add(DEADLINE_TIME)
                .unwrap_or_revert()
                < U256::from(u64::from(get_blocktime()))
    }

    ///  @dev returns true total collected is below the min asked
    fn below_floor_asked(&self) -> bool {
        get_total_collected() < get_floor_asked()
    }

    /// @dev returns true if nextDueTime is 0, mean it has not been initialized (unix timestamp)
    fn payment_time_not_set(&self) -> bool {
        get_next_due_time() == 0.into()
    }

    /// @dev returns true if contract is in contribution phase time window
    fn contribution_phase(&self) -> bool {
        self.time_since(get_creation_time()) < CONTRIBUTION_TIME
    }

    /// @dev returns final due time of loan
    fn payback_timestamp(&self) -> U256 {
        self.starting_timestamp()
            .checked_add(get_globals().payment_time)
            .unwrap_or_revert()
    }

    /// @dev returns approximate time the loan will/did start
    fn starting_timestamp(&self) -> U256 {
        get_creation_time()
            .checked_add(CONTRIBUTION_TIME)
            .unwrap_or_revert()
    }

    /// @dev returns address to transfer NFT to in event of liquidation
    fn liquidate_to(&self) -> Key {
        if get_single_provider() == zero_address()
            || get_single_provider() == account_zero_address()
        {
            get_trustee_multisig()
        } else {
            get_single_provider()
        }
    }

    /// @dev returns bool if owner was removed
    fn ownerless_locker(&self) -> bool {
        get_globals().locker_owner == zero_address()
            || get_globals().locker_owner == account_zero_address()
    }

    /// @dev returns calc of time since a certain timestamp to block timestamp
    fn time_since(&self, time_stamp: U256) -> U256 {
        U256::from(u64::from(get_blocktime()))
            .checked_sub(time_stamp)
            .unwrap_or_revert_with(Error::LiquidHelperUnderflowSub0)
    }

    /// @dev sets due time to 0
    fn _revoke_due_time(&self) {
        set_next_due_time(0.into());
    }

    /// @dev adds a contribution on to the currently stored amount of contributions for a user
    fn _increase_contributions(&self, contributors_address: Key, contribution_amount: U256) {
        Contributions::instance().set(
            &contributors_address,
            Contributions::instance()
                .get(&contributors_address)
                .checked_add(contribution_amount)
                .unwrap_or_revert(),
        )
    }

    /// @dev adds an amount to totalCollected
    fn _increase_total_collected(&self, increase_amount: U256) {
        set_total_collected(
            get_total_collected()
                .checked_add(increase_amount)
                .unwrap_or_revert(),
        );
    }

    /// @dev subs an amount to totalCollected
    fn _decrease_total_collected(&self, decrease_amount: U256) {
        set_total_collected(
            get_total_collected()
                .checked_sub(decrease_amount)
                .unwrap_or_revert_with(Error::LiquidHelperUnderflowSub1),
        );
    }

    /// @dev Helper function to add payment tokens and penalty tokens to their internal variables
    /// Also calculates remainingBalance due for the owner.
    fn _adjust_balances(&self, payment_tokens: U256, penalty_tokens: U256) {
        set_claimable_balance(
            get_claimable_balance()
                .checked_add(payment_tokens)
                .unwrap_or_revert(),
        );
        let new_balance = get_remaining_balance()
            .checked_add(penalty_tokens)
            .unwrap_or_revert();
        set_remaining_balance({
            if payment_tokens < new_balance {
                new_balance.checked_sub(payment_tokens).unwrap_or_revert()
            } else {
                0.into()
            }
        });
    }

    fn _safe_transfer(&self, token: Key, recipient: Key, amount: U256) {
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount
            },
        );
        ret.unwrap_or_revert();
    }

    fn _safe_transfer_from(&self, token: Key, owner: Key, recipient: Key, amount: U256) {
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "transfer_from",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "amount" => amount
            },
        );
        ret.unwrap_or_revert();
    }
}
