use crate::data;
use alloc::vec::Vec;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use liquid_base_crate::{self, data::*, LIQUIDBASE};

use common::errors::*;

pub trait LIQUIDHELPER<Storage: ContractStorage>:
    ContractContext<Storage> + LIQUIDBASE<Storage>
{
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash) {
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);

        LIQUIDBASE::init(self);
    }

    fn get_tokens(&self) -> Vec<U256> {
        get_globals().token_id
    }

    fn ownerless_locker(&self) -> bool {
        let locker_owner: Key = get_globals().locker_owner;
        locker_owner == zero_address()
    }

    fn floor_not_reached(&self) -> bool {
        !self.contribution_phase() && self.below_floor_asked()
    }

    fn not_single_provider(&self, check_address: Key) -> bool {
        LIQUIDBASE::get_single_provider(self) != check_address
            && LIQUIDBASE::get_single_provider(self) != zero_address()
    }

    fn reached_total(&self, contributor: Key, token_amount: U256) -> bool {
        let ans: U256 = LIQUIDBASE::Contributions(self)
            .get(&contributor)
            .checked_add(token_amount)
            .unwrap_or_revert();
        ans >= LIQUIDBASE::get_total_asked(self)
    }

    fn missed_activate(&self) -> bool {
        let blocktime: u64 = runtime::get_blocktime().into();
        let sum: U256 = self
            .starting_timestamp()
            .checked_add(DEADLINE_TIME)
            .unwrap_or_revert();
        self.payment_time_not_set() && sum < U256::from(blocktime)
    }

    fn missed_deadline(&self) -> bool {
        let blocktime: u64 = runtime::get_blocktime().into();
        let sum: U256 = LIQUIDBASE::get_next_due_time(self)
            .checked_add(DEADLINE_TIME)
            .unwrap_or_revert();
        LIQUIDBASE::get_next_due_time(self) > 0.into() && sum < U256::from(blocktime)
    }

    fn below_floor_asked(&self) -> bool {
        LIQUIDBASE::get_total_collected(self) < LIQUIDBASE::get_floor_asked(self)
    }

    fn payment_time_not_set(&self) -> bool {
        LIQUIDBASE::get_next_due_time(self) == 0.into()
    }

    fn contribution_phase(&self) -> bool {
        self.time_since(LIQUIDBASE::get_creation_time(self)) < CONTRIBUTION_TIME
    }

    fn payback_timestamp(&self) -> U256 {
        let sum: U256 = self
            .starting_timestamp()
            .checked_add(get_globals().payment_time)
            .unwrap_or_revert();
        sum
    }

    fn starting_timestamp(&self) -> U256 {
        let sum: U256 = LIQUIDBASE::get_creation_time(self)
            .checked_add(CONTRIBUTION_TIME)
            .unwrap_or_revert();
        sum
    }

    fn liquidate_to(&self) -> Key {
        if LIQUIDBASE::get_single_provider(self) == zero_address() {
            get_trustee_multisig()
        } else {
            LIQUIDBASE::get_single_provider(self)
        }
    }

    fn time_since(&self, time_stamp: U256) -> U256 {
        let blocktime: u64 = runtime::get_blocktime().into();
        let sub: U256 = U256::from(blocktime)
            .checked_sub(time_stamp)
            .unwrap_or_revert_with(Error::LiquidHelperUnderflowSub0);
        sub
    }

    fn _add(&self, _a: U256, _b: U256) -> U256 {
        let sum: U256 = _a.checked_add(_b).unwrap_or_revert();
        sum
    }

    fn _revoke_owner(&self) {
        let mut g = get_globals();
        g.locker_owner = zero_address();
        set_globals(g);
    }

    fn _revoke_due_time(&self) {
        LIQUIDBASE::set_next_due_time(self, 0.into());
    }

    fn _increase_contributions(&self, contributors_address: Key, contribution_amount: U256) {
        LIQUIDBASE::Contributions(self).set(
            &contributors_address,
            LIQUIDBASE::Contributions(self)
                .get(&contributors_address)
                .checked_add(contribution_amount)
                .unwrap_or_revert(),
        )
    }

    fn _increase_total_collected(&self, increase_amount: U256) {
        let sum: U256 = LIQUIDBASE::get_total_collected(self)
            .checked_add(increase_amount)
            .unwrap_or_revert();
        LIQUIDBASE::set_total_collected(self, sum);
    }

    fn _decrease_total_collected(&self, decrease_amount: U256) {
        let sub: U256 = LIQUIDBASE::get_total_collected(self)
            .checked_sub(decrease_amount)
            .unwrap_or_revert_with(Error::LiquidHelperUnderflowSub1);
        LIQUIDBASE::set_total_collected(self, sub);
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
