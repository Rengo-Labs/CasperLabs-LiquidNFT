use crate::data;
use alloc::vec::Vec;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, U256};
use contract_utils::{ContractContext, ContractStorage};
use liquid_base_crate::{
    self,
    data::{CONTRIBUTION_TIME, DEADLINE_TIME},
    LIQUIDBASE,
};
use liquid_nft_utils::commons::key_names::*;

#[repr(u16)]
pub enum Error {
    TransferFailed = 65,
    TransferFromFailed = 66,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait LIQUIDHELPER<Storage: ContractStorage>:
    ContractContext<Storage> + LIQUIDBASE<Storage>
{
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash) {
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);

        LIQUIDBASE::init(self);
    }

    fn get_tokens(&self) -> Vec<U256> {
        LIQUIDBASE::Globals(self).get(TOKEN_ID)
    }

    fn ownerless_locker(&self) -> bool {
        let locker_owner: Key = LIQUIDBASE::Globals(self).get(LOCKER_OWNER);
        locker_owner == LIQUIDBASE::ZERO_ADDRESS(self)
    }

    fn floor_not_reached(&self) -> bool {
        self.contribution_phase() == false && self.below_floor_asked() == true
    }

    fn not_single_provider(&self, check_address: Key) -> bool {
        LIQUIDBASE::get_single_provider(self) != check_address
            && LIQUIDBASE::get_single_provider(self) != LIQUIDBASE::ZERO_ADDRESS(self)
    }

    fn reached_total(&self, contributor: Key, token_amount: U256) -> bool {
        LIQUIDBASE::Contributions(self)
            .get(&contributor)
            .checked_add(token_amount)
            .unwrap_or_revert()
            >= LIQUIDBASE::get_total_asked(self)
    }

    fn missed_activate(&self) -> bool {
        let blocktime: u64 = runtime::get_blocktime().into();
        self.payment_time_not_set()
            && self
                .starting_timestamp()
                .checked_add(U256::from(DEADLINE_TIME))
                .unwrap_or_revert()
                < U256::from(blocktime)
    }

    fn missed_deadline(&self) -> bool {
        let blocktime: u64 = runtime::get_blocktime().into();
        LIQUIDBASE::get_next_due_time(self) > 0.into()
            && LIQUIDBASE::get_next_due_time(self)
                .checked_add(U256::from(DEADLINE_TIME))
                .unwrap_or_revert()
                < U256::from(blocktime)
    }

    fn below_floor_asked(&self) -> bool {
        LIQUIDBASE::get_total_collected(self) < LIQUIDBASE::get_floor_asked(self)
    }

    fn payment_time_not_set(&self) -> bool {
        LIQUIDBASE::get_next_due_time(self) == 0.into()
    }

    fn contribution_phase(&self) -> bool {
        self.time_since(LIQUIDBASE::get_creation_time(self)) < U256::from(CONTRIBUTION_TIME)
    }

    fn payback_timestamp(&self) -> U256 {
        self.starting_timestamp()
            .checked_add(LIQUIDBASE::Globals(self).get(PAYMENT_TIME))
            .unwrap_or_revert()
    }

    fn starting_timestamp(&self) -> U256 {
        LIQUIDBASE::get_creation_time(self)
            .checked_add(U256::from(CONTRIBUTION_TIME))
            .unwrap_or_revert()
    }

    fn liquidate_to(&self) -> Key {
        if LIQUIDBASE::get_single_provider(self) == LIQUIDBASE::ZERO_ADDRESS(self) {
            LIQUIDBASE::TRUSTEE_MULTISIG(self)
        } else {
            LIQUIDBASE::get_single_provider(self)
        }
    }

    fn time_since(&self, time_stamp: U256) -> U256 {
        let blocktime: u64 = runtime::get_blocktime().into();
        U256::from(blocktime)
            .checked_sub(time_stamp)
            .unwrap_or_revert()
    }

    fn _add(&self, _a: U256, _b: U256) -> U256 {
        _a.checked_add(_b).unwrap_or_revert()
    }

    fn _revoke_owner(&self) {
        LIQUIDBASE::Globals(self).set(LOCKER_OWNER, LIQUIDBASE::ZERO_ADDRESS(self));
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
        LIQUIDBASE::set_total_collected(
            self,
            LIQUIDBASE::get_total_collected(self)
                .checked_add(increase_amount)
                .unwrap_or_revert(),
        );
    }

    fn _decrease_total_collected(&self, decrease_amount: U256) {
        LIQUIDBASE::set_total_collected(
            self,
            LIQUIDBASE::get_total_collected(self)
                .checked_sub(decrease_amount)
                .unwrap_or_revert(),
        );
    }

    fn _safe_transfer(&self, token: Key, to: Key, value: U256) {
        let ret: Result<(), u32> = runtime::call_contract(
            token.into_hash().unwrap_or_revert().into(),
            "transfer",
            runtime_args! {
                "to" => to,
                "value" => value
            },
        );

        if ret.is_err() {
            runtime::revert(ApiError::from(Error::TransferFailed));
        }
    }

    fn _safe_transfer_from(token: Key, from: Key, to: Key, value: U256) {
        let ret: Result<(), u32> = runtime::call_contract(
            token.into_hash().unwrap_or_revert().into(),
            "transfer",
            runtime_args! {
                "from" => from,
                "to" => to,
                "value" => value
            },
        );

        if ret.is_err() {
            runtime::revert(ApiError::from(Error::TransferFromFailed));
        }
    }
}
