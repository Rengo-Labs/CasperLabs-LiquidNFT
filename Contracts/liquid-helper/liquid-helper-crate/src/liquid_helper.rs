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
    ///@dev returns IDs of NFTs being held
    fn get_tokens(&self) -> Vec<U256> {
        get_globals().token_id
    }
    fn floor_not_reached(&self) -> bool {
        !self.contribution_phase() && self.below_floor_asked()
    }
    ///@dev returns true if the provider address is not the single provider
    fn not_single_provider(&self, check_address: Key) -> bool {
        get_single_provider() != check_address && get_single_provider() != zero_address()
    }
    ///@dev returns true if the contributor will reach the ceiling asked with the provided token amount
    fn reached_total(&self, contributor: Key, token_amount: U256) -> bool {
        Contributions::instance()
            .get(&contributor)
            .checked_add(token_amount)
            .unwrap_or_revert()
            >= get_total_asked()
    }
    ///@dev returns true if locker has not been enabled within 7 days after contribution phase
    fn missed_activate(&self) -> bool {
        let blocktime: u64 = runtime::get_blocktime().into();
        let sum: U256 = self
            .starting_timestamp()
            .checked_add(DEADLINE_TIME)
            .unwrap_or_revert();
        self.payment_time_not_set() && sum < U256::from(blocktime)
    }
    ///@dev returns true if owner has not paid back within 7 days of last payment
    fn missed_deadline(&self) -> bool {
        let blocktime: u64 = runtime::get_blocktime().into();
        let sum: U256 = get_next_due_time()
            .checked_add(DEADLINE_TIME)
            .unwrap_or_revert();
        get_next_due_time() > 0.into() && sum < U256::from(blocktime)
    }
    /// @dev returns true total collected is below the min asked
    fn below_floor_asked(&self) -> bool {
        get_total_collected() < get_floor_asked()
    }
    ///@dev returns true if nextDueTime is 0, mean it has not been initialized (unix timestamp)
    fn payment_time_not_set(&self) -> bool {
        get_next_due_time() == 0.into()
    }
    ///@dev returns true if contract is in contribution phase time window
    fn contribution_phase(&self) -> bool {
        self.time_since(get_creation_time()) < CONTRIBUTION_TIME
    }
    ///@dev returns final due time of loan
    fn payback_timestamp(&self) -> U256 {
        let sum: U256 = self
            .starting_timestamp()
            .checked_add(get_globals().payment_time)
            .unwrap_or_revert();
        sum
    }
    ///@dev returns approximate time the loan will/did start
    fn starting_timestamp(&self) -> U256 {
        let sum: U256 = get_creation_time()
            .checked_add(CONTRIBUTION_TIME)
            .unwrap_or_revert();
        sum
    }
    ///@dev returns address to transfer NFT to in event of liquidation
    fn liquidate_to(&self) -> Key {
        if get_single_provider() == zero_address() {
            get_trustee_multisig()
        } else {
            get_single_provider()
        }
    }

    fn ownerless_locker(&self) -> bool {
        let locker_owner: Key = get_globals().locker_owner;
        locker_owner == zero_address()
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
        set_next_due_time(0.into());
    }

    fn _increase_contributions(&self, contributors_address: Key, contribution_amount: U256) {
        Contributions::instance().set(
            &contributors_address,
            Contributions::instance()
                .get(&contributors_address)
                .checked_add(contribution_amount)
                .unwrap_or_revert(),
        )
    }

    fn _increase_total_collected(&self, increase_amount: U256) {
        let sum: U256 = get_total_collected()
            .checked_add(increase_amount)
            .unwrap_or_revert();
        set_total_collected(sum);
    }

    fn _decrease_total_collected(&self, decrease_amount: U256) {
        let sub: U256 = get_total_collected()
            .checked_sub(decrease_amount)
            .unwrap_or_revert_with(Error::LiquidHelperUnderflowSub1);
        set_total_collected(sub);
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
