use crate::data;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256};
use contract_utils::{ContractContext, ContractStorage};
use liquid_transfer_crate::LiquidTransfer;
use renvm_sig::keccak256;

use data::*;

#[repr(u16)]
pub enum Error {
    LiquidFactoryInvalidMaster = 0,
    LiquidFactoryInvalidLocker,
    LiquidFactoryTransferFromFailed,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub enum LiquidFactoryEvent {
    NewLocker {
        locker_address: Key,
        owners_address: Key,
        tokens_address: Key,
    },
    ContributeToLocker {
        locker_address: Key,
        backer_address: Key,
        contribution_amount: U256,
        total_increase_amount: U256,
    },
    DonateToLocker {
        locker_address: Key,
        payers_address: Key,
        donate_amount: U256,
    },
    PaybackToLocker {
        locker_address: Key,
        payers_address: Key,
        payback_amount: U256,
    },
    NewEmptyLocker {
        locker_address: Key,
    },
    ReusedLocker {
        locker_address: Key,
    },
}

impl LiquidFactoryEvent {
    pub fn type_name(&self) -> String {
        match self {
            LiquidFactoryEvent::NewLocker {
                locker_address: _,
                owners_address: _,
                tokens_address: _,
            } => "newLocker",
            LiquidFactoryEvent::ContributeToLocker {
                locker_address: _,
                backer_address: _,
                contribution_amount: _,
                total_increase_amount: _,
            } => "contributeToLocker",
            LiquidFactoryEvent::DonateToLocker {
                locker_address: _,
                payers_address: _,
                donate_amount: _,
            } => "donateToLocker",
            LiquidFactoryEvent::PaybackToLocker {
                locker_address: _,
                payers_address: _,
                payback_amount: _,
            } => "paybackToLocker",
            LiquidFactoryEvent::NewEmptyLocker { locker_address: _ } => "newEmptyLocker",
            LiquidFactoryEvent::ReusedLocker { locker_address: _ } => "reusedLocker",
        }
        .to_string()
    }
}

pub trait LIQUIDFACTORY<Storage: ContractStorage>:
    ContractContext<Storage> + LiquidTransfer<Storage>
{
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
        data::set_default_target(default_target);
        data::set_master_address(self.get_caller());
        data::set_default_target(data::get_zero_address());
        data::set_locker_count(0.into());
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        Implementations::init();
        Lockers::init();

        Implementations::instance().set(&default_token, default_target);

        self._store_predictions(data::get_locker_count(), default_count, default_token);

        self._generate_locker(default_token);

        // TEMPORARY
        let punks = data::get_zero_address();
        let kitties = data::get_zero_address();

        // INIT CRATES
        LiquidTransfer::init(self, punks, kitties, contract_hash, package_hash);
    }

    fn only_master(&self) {
        if self.get_caller().to_formatted_string()
            != data::get_master_address().to_formatted_string()
        {
            runtime::revert(ApiError::from(Error::LiquidFactoryInvalidMaster));
        }
    }

    fn _store_predictions(
        &self,
        prediction_start: U256,
        prediction_count: U256,
        prediction_token: Key,
    ) {
        let mut i: U256 = prediction_start;
        while i < prediction_count {
            let predicted = self.predict_locker_address(
                i,
                data::get_hash(),
                self.get_implementation(prediction_token),
            );
            Lockers::instance().set(&predicted, true);
            i = i.checked_add(1.into()).unwrap_or_revert();
        }
    }

    fn store_predictions(
        &self,
        prediction_start: U256,
        prediction_count: U256,
        prediction_token: Key,
    ) {
        self._store_predictions(prediction_start, prediction_count, prediction_token);
    }

    fn predict_locker_address(&self, index: U256, factory: Key, implementation: Key) -> Key {
        let salt = keccak256(&hex::encode(index.to_string()).into_bytes());

        // assembly {
        //     let ptr := mload(0x40)
        //     mstore(ptr, 0x3d602d80600a3d3981f3363d3d373d3d3d363d73000000000000000000000000)
        //     mstore(add(ptr, 0x14), shl(0x60, _implementation))
        //     mstore(add(ptr, 0x28), 0x5af43d82803e903d91602b57fd5bf3ff00000000000000000000000000000000)
        //     mstore(add(ptr, 0x38), shl(0x60, _factory))
        //     mstore(add(ptr, 0x4c), salt)
        //     mstore(add(ptr, 0x6c), keccak256(ptr, 0x37))
        //     predicted := keccak256(add(ptr, 0x37), 0x55)
        // }

        // TEMPORARY
        data::get_zero_address()
    }

    fn _generate_locker(&self, payment_token: Key) -> Key {
        let salt = keccak256(&hex::encode(data::get_locker_count().to_string()).into_bytes());
        data::set_locker_count(data::get_locker_count() + 1);

        //     bytes20 targetBytes = bytes20(
        //         getImplementation(_paymentToken)
        //     );

        //     assembly {

        //         let clone := mload(0x40)

        //         mstore(
        //             clone,
        //             0x3d602d80600a3d3981f3363d3d373d3d3d363d73000000000000000000000000
        //         )

        //         mstore(
        //             add(clone, 0x14),
        //             targetBytes
        //         )

        //         mstore(
        //             add(clone, 0x28),
        //             0x5af43d82803e903d91602b57fd5bf30000000000000000000000000000000000
        //         )

        //         lockerAddress := create2(0, clone, 0x37, salt)
        //     }

        //     if (lockers[lockerAddress] == false) {
        //         lockers[lockerAddress] = true;
        //     }

        // TEMPORARY
        data::get_zero_address()
    }

    fn update_default_target(&self, new_default_target: Key) {
        self.only_master();
        data::set_default_target(new_default_target);
    }

    fn update_implementation(&self, token_address: Key, target_address: Key) {
        self.only_master();
        Implementations::instance().set(&token_address, target_address);
    }

    fn update_master(&self, new_master: Key) {
        self.only_master();
        data::set_master_address(new_master);
    }

    fn revoke_master(&self) {
        self.only_master();
        data::set_master_address(data::get_zero_address());
    }

    fn create_liquid_locker(
        &mut self,
        token_id: Vec<U256>,
        token_address: Key,
        floor_asked: U256,
        total_asked: U256,
        payment_time: U256,
        payment_rate: U256,
        payment_token: Key,
    ) -> Key {
        let locker_address = self._generate_locker(payment_token);

        let () = runtime::call_contract(
            locker_address.into_hash().unwrap_or_revert().into(),
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

        let mut i: U256 = 0.into();
        while i < U256::from(token_id.len()) {
            LiquidTransfer::transfer_from_nft(
                self,
                self.get_caller(),
                locker_address,
                token_address,
                token_id[i.as_usize()],
            );
            i = i.checked_add(1.into()).unwrap_or_revert();
        }

        self.liquid_factory_emit(&LiquidFactoryEvent::NewLocker {
            locker_address,
            owners_address: self.get_caller(),
            tokens_address: token_address,
        });

        locker_address
    }

    fn create_empty_locker(&mut self, payment_token: Key) -> Key {
        let locker_address = self._generate_locker(payment_token);

        self.liquid_factory_emit(&LiquidFactoryEvent::NewEmptyLocker { locker_address });

        locker_address
    }

    fn contribute_to_locker(&mut self, lockers_address: Key, payment_amount: U256) -> (U256, U256) {
        if Lockers::instance().get(&lockers_address) != true {
            runtime::revert(ApiError::from(Error::LiquidFactoryInvalidLocker));
        }

        let (total_increase, users_increase): (U256, U256) = runtime::call_contract(
            lockers_address.into_hash().unwrap_or_revert().into(),
            "make_contribution",
            runtime_args! {
                "token_amount" => payment_amount,
                "token_holder" => self.get_caller()
            },
        );

        let payment_token: Key = runtime::call_contract(
            lockers_address.into_hash().unwrap_or_revert().into(),
            "PAYMENT_TOKEN",
            runtime_args! {},
        );
        self._safe_transfer_from(
            payment_token,
            self.get_caller(),
            lockers_address,
            users_increase,
        );

        self.liquid_factory_emit(&LiquidFactoryEvent::ContributeToLocker {
            locker_address: lockers_address,
            backer_address: self.get_caller(),
            contribution_amount: users_increase,
            total_increase_amount: total_increase,
        });

        (total_increase, users_increase)
    }

    fn get_implementation(&self, payment_token: Key) -> Key {
        let implementation: Key;
        if Implementations::instance()
            .get(&payment_token)
            .to_formatted_string()
            == data::get_zero_address().to_formatted_string()
        {
            implementation = data::get_default_target();
        } else {
            implementation = Implementations::instance().get(&payment_token);
        }
        implementation
    }

    fn donate_to_locker(&mut self, lockers_address: Key, donation_amount: U256) {
        if Lockers::instance().get(&lockers_address) == true {
            runtime::revert(ApiError::from(Error::LiquidFactoryInvalidLocker));
        }

        let () = runtime::call_contract(
            lockers_address.into_hash().unwrap_or_revert().into(),
            "donate_funds",
            runtime_args! {
                "donation_amount" => donation_amount
            },
        );

        let payment_token: Key = runtime::call_contract(
            lockers_address.into_hash().unwrap_or_revert().into(),
            "PAYMENT_TOKEN",
            runtime_args! {},
        );
        self._safe_transfer_from(
            payment_token,
            self.get_caller(),
            lockers_address,
            donation_amount,
        );

        self.liquid_factory_emit(&LiquidFactoryEvent::DonateToLocker {
            locker_address: lockers_address,
            payers_address: self.get_caller(),
            donate_amount: donation_amount,
        });
    }

    fn payback_to_locker(&mut self, lockers_address: Key, payment_amount: U256) {
        if Lockers::instance().get(&lockers_address) != true {
            runtime::revert(ApiError::from(Error::LiquidFactoryInvalidLocker));
        }

        let () = runtime::call_contract(
            lockers_address.into_hash().unwrap_or_revert().into(),
            "pay_back_funds",
            runtime_args! {
                "payment_amount" => payment_amount
            },
        );

        let payment_token: Key = runtime::call_contract(
            lockers_address.into_hash().unwrap_or_revert().into(),
            "PAYMENT_TOKEN",
            runtime_args! {},
        );
        self._safe_transfer_from(
            payment_token,
            self.get_caller(),
            lockers_address,
            payment_amount,
        );

        self.liquid_factory_emit(&LiquidFactoryEvent::PaybackToLocker {
            locker_address: lockers_address,
            payers_address: self.get_caller(),
            payback_amount: payment_amount,
        });
    }

    fn _safe_transfer_from(&self, token: Key, from: Key, to: Key, value: U256) {
        let ret: Result<(), u32> = runtime::call_contract(
            token.into_hash().unwrap_or_revert().into(),
            "transfer_from",
            runtime_args! {
                "from" => from,
                "to" => to,
                "value" => value
            },
        );

        if ret.is_err() {
            runtime::revert(ApiError::from(Error::LiquidFactoryTransferFromFailed));
        }
    }

    fn liquid_factory_emit(&mut self, liquid_factory_event: &LiquidFactoryEvent) {
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
            LiquidFactoryEvent::ReusedLocker { locker_address } => {
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
