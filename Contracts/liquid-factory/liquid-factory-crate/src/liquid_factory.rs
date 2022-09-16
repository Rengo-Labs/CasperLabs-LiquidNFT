use crate::{data, events::LiquidFactoryEvent};
use alloc::{
    collections::{BTreeMap, BTreeSet},
    format,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::errors::*;
use liquid_locker_crate::{
    entry_points::get_entry_points, liquid_helper_crate::liquid_base_crate, LIQUIDLOCKER,
};
use liquid_transfer_crate::LIQUIDTRANSFER;

use data::*;

pub trait LIQUIDFACTORY<Storage: ContractStorage>:
    ContractContext<Storage> + LIQUIDTRANSFER<Storage> + LIQUIDLOCKER<Storage>
{
    /// @dev Set parameters and precompute some locker addresses.
    fn init(
        &mut self,
        default_count: U256,
        default_token: Key,
        default_target: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        data::set_default_count(default_count);
        data::set_default_token(default_token);
        data::set_master_address(self.get_caller());
        data::set_locker_count(0.into());

        Implementations::init();
        Lockers::init();
        Implementations::instance().set(&default_token, default_target);

        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);

        self._generate_locker(default_token);
    }

    fn only_master(&self) {
        if self.get_caller().to_formatted_string()
            != data::get_master_address().to_formatted_string()
        {
            runtime::revert(ApiError::from(Error::InvalidMaster));
        }
    }

    fn is_locker(&self, lockers_address: &Key) {
        if !Lockers::instance().get(lockers_address) {
            runtime::revert(ApiError::from(Error::InvalidLocker));
        }
    }

    /// @dev Clone the byte code from one contract into a new contract. Uses solidity assembly.
    /// This is a lot cheaper in gas than deploying a new contract.
    fn _generate_locker(&self, payment_token: Key) -> (Key, Key) {
        // Factory
        let salt: String = data::get_counter().to_string();
        data::set_counter(data::get_counter().checked_add(1.into()).unwrap_or_revert());
        let name: String = "Locker-".to_string() + &salt;
        let (package_hash, _) = storage::create_contract_package_at_hash();
        let (contract_hash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());
        runtime::put_key(&format!("{}_contract", name), contract_hash.into());

        // Making constructor access to call
        let constructor_access: URef =
            storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert();

        // Call the constructor entry point
        let _: () = runtime::call_versioned_contract(
            package_hash,
            None,
            "constructor",
            runtime_args! {
                "trustee_multisig" => liquid_base_crate::data::get_trustee_multisig(),
                "payment_token" => payment_token,
                "package_hash" => package_hash,
                "contract_hash" => contract_hash,
            },
        );

        // Remove all URefs from the constructor group, so no one can call it for the second time.
        let mut urefs = BTreeSet::new();
        urefs.insert(constructor_access);
        storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
            .unwrap_or_revert();

        if !Lockers::instance().get(&Key::from(package_hash)) {
            Lockers::instance().set(&Key::from(package_hash), true);
        }

        // New deployed locker address
        (Key::from(contract_hash), Key::from(package_hash))
    }

    /// @dev Transfer master permission
    fn update_master(&self, new_master: Key) {
        self.only_master();
        data::set_master_address(new_master);
    }
    /// @dev Destroy Master functionality
    fn revoke_master(&self) {
        self.only_master();
        data::set_master_address(data::zero_address());
    }

    /// Call into initialize for the locker to begin the LiquidNFT loan process.
    /// Transfer the NFT the user wants use for the loan into the locker.
    #[allow(clippy::too_many_arguments)]
    fn create_liquid_locker(
        &mut self,
        token_id: Vec<U256>,
        token_address: Key,
        floor_asked: U256,
        total_asked: U256,
        payment_time: U256,
        payment_rate: U256,
        payment_token: Key,
    ) -> (Key, Key) {
        let (locker_contract_address, locker_package_address) =
            self._generate_locker(payment_token);
        let () = runtime::call_versioned_contract(
            locker_package_address.into_hash().unwrap_or_revert().into(),
            None,
            "initialize",
            runtime_args! {
                "token_id" => token_id.clone(),
                "token_address" => token_address,
                "token_owner" => self.get_caller(),
                "floor_asked" => floor_asked,
                "total_asked" => total_asked,
                "payment_time" => payment_time,
                "payment_rate" => payment_rate
            },
        );
        LIQUIDTRANSFER::transfer_from_nft(
            self,
            self.get_caller(),
            locker_package_address,
            token_address,
            token_id,
        );
        LIQUIDFACTORY::emit(
            self,
            &LiquidFactoryEvent::NewLocker {
                locker_address: locker_package_address,
                owners_address: self.get_caller(),
                tokens_address: token_address,
            },
        );
        (locker_contract_address, locker_package_address)
    }

    /// Creating an empty locker without any liquidity
    fn create_empty_locker(&mut self, payment_token: Key) -> (Key, Key) {
        let (locker_contract_address, locker_package_address) =
            self._generate_locker(payment_token);
        LIQUIDFACTORY::emit(
            self,
            &LiquidFactoryEvent::NewEmptyLocker {
                locker_address: locker_package_address,
            },
        );
        (locker_contract_address, locker_package_address)
    }

    /// @dev Call contributeToLocker. Factory acts as a middle man between the user and the locker.
    /// We do this so that the user only has to approve the factory and not each new locker.
    fn contribute_to_locker(&mut self, lockers_address: Key, payment_amount: U256) -> (U256, U256) {
        self.is_locker(&lockers_address);
        let (total_increase, users_increase): (U256, U256) = runtime::call_versioned_contract(
            lockers_address.into_hash().unwrap_or_revert().into(),
            None,
            "make_contribution",
            runtime_args! {
                "token_amount" => payment_amount,
                "token_holder" => self.get_caller()
            },
        );
        let payment_token: Key = runtime::call_versioned_contract(
            lockers_address.into_hash().unwrap_or_revert().into(),
            None,
            "payment_token",
            runtime_args! {},
        );
        self._safe_transfer_from(
            payment_token,
            self.get_caller(),
            lockers_address,
            users_increase,
        );
        LIQUIDFACTORY::emit(
            self,
            &LiquidFactoryEvent::ContributeToLocker {
                locker_address: lockers_address,
                backer_address: self.get_caller(),
                contribution_amount: users_increase,
                total_increase_amount: total_increase,
            },
        );
        (total_increase, users_increase)
    }

    /// dev Give tokens to a locker. These tokens do not go toward paying off the loan,
    /// they are instead distributed among the contributors for the loan.
    /// The result of this is that the value is transferred to the contributors not the owner because it does
    /// not deduct from the balance the owner owes.
    fn donate_to_locker(&mut self, lockers_address: Key, donation_amount: U256) {
        self.is_locker(&lockers_address);
        let () = runtime::call_versioned_contract(
            lockers_address.into_hash().unwrap_or_revert().into(),
            None,
            "donate_funds",
            runtime_args! {
                "donation_amount" => donation_amount
            },
        );
        let payment_token: Key = runtime::call_versioned_contract(
            lockers_address.into_hash().unwrap_or_revert().into(),
            None,
            "payment_token",
            runtime_args! {},
        );
        self._safe_transfer_from(
            payment_token,
            self.get_caller(),
            lockers_address,
            donation_amount,
        );
        LIQUIDFACTORY::emit(
            self,
            &LiquidFactoryEvent::DonateToLocker {
                locker_address: lockers_address,
                payers_address: self.get_caller(),
                donate_amount: donation_amount,
            },
        );
    }

    /// @dev Call paybackToLocker. Factory acts as a middle man between the user and the locker.
    /// We do this so that the user only has to approve the factory and not each new locker.
    fn payback_to_locker(&mut self, lockers_address: Key, payment_amount: U256) {
        self.is_locker(&lockers_address);
        let () = runtime::call_versioned_contract(
            lockers_address.into_hash().unwrap_or_revert().into(),
            None,
            "pay_back_funds",
            runtime_args! {
                "payment_amount" => payment_amount
            },
        );
        let payment_token: Key = runtime::call_versioned_contract(
            lockers_address.into_hash().unwrap_or_revert().into(),
            None,
            "payment_token",
            runtime_args! {},
        );
        self._safe_transfer_from(
            payment_token,
            self.get_caller(),
            lockers_address,
            payment_amount,
        );
        LIQUIDFACTORY::emit(
            self,
            &LiquidFactoryEvent::PaybackToLocker {
                locker_address: lockers_address,
                payers_address: self.get_caller(),
                payback_amount: payment_amount,
            },
        );
    }

    fn emit(&mut self, liquid_factory_event: &LiquidFactoryEvent) {
        let mut events = Vec::new();
        let package = data::get_contract_package_hash();
        match liquid_factory_event {
            LiquidFactoryEvent::NewLocker {
                locker_address,
                owners_address,
                tokens_address,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_factory_event.type_name());
                event.insert("locker_address", locker_address.to_string());
                event.insert("owners_address", owners_address.to_string());
                event.insert("tokens_address", tokens_address.to_string());
                events.push(event);
            }
            LiquidFactoryEvent::ContributeToLocker {
                locker_address,
                backer_address,
                contribution_amount,
                total_increase_amount,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_factory_event.type_name());
                event.insert("locker_address", locker_address.to_string());
                event.insert("backer_address", backer_address.to_string());
                event.insert("contribution_amount", contribution_amount.to_string());
                event.insert("total_increase_amount", total_increase_amount.to_string());
                events.push(event);
            }
            LiquidFactoryEvent::DonateToLocker {
                locker_address,
                payers_address,
                donate_amount,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_factory_event.type_name());
                event.insert("locker_address", locker_address.to_string());
                event.insert("payers_address", payers_address.to_string());
                event.insert("donate_amount", donate_amount.to_string());
                events.push(event);
            }
            LiquidFactoryEvent::PaybackToLocker {
                locker_address,
                payers_address,
                payback_amount,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_factory_event.type_name());
                event.insert("locker_address", locker_address.to_string());
                event.insert("payers_address", payers_address.to_string());
                event.insert("payback_amount", payback_amount.to_string());
                events.push(event);
            }
            LiquidFactoryEvent::NewEmptyLocker { locker_address } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_factory_event.type_name());
                event.insert("locker_address", locker_address.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
