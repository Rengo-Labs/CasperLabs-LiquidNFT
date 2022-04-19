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

#[repr(u16)]
pub enum Error {
    NftTransferFailed = 0,
    InvalidOwner = 1,
    BuyPunkResult = 2,
    TransferError = 3,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub enum LiquidTransferEvent {
    ERC721Received {
        operator: Key,
        from: Key,
        token_id: U256,
        data: String,
    },
}

impl LiquidTransferEvent {
    pub fn type_name(&self) -> String {
        match self {
            LiquidTransferEvent::ERC721Received {
                operator: _,
                from: _,
                token_id: _,
                data: _,
            } => "erc721Received",
        }
        .to_string()
    }
}

pub trait LIQUIDTRANSFER<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        punks: Key,
        kitties: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        data::set_punks(punks);
        data::set_kitties(kitties);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
    }

    // Checks if contract is nonstandard, does transfer according to contract implementation
    fn transfer_nft(&self, from: Key, to: Key, token_address: Key, token_id: U256) {
        let success: bool;
        if token_address.to_formatted_string() == data::get_kitties().to_formatted_string() {
            success = runtime::call_contract(
                token_address.into_hash().unwrap_or_revert().into(),
                "transfer",
                runtime_args! {
                    "to" => to,
                    "token_id" => token_id
                },
            );
        } else if token_address.to_formatted_string() == data::get_punks().to_formatted_string() {
            success = runtime::call_contract(
                token_address.into_hash().unwrap_or_revert().into(),
                "transfer_punk",
                runtime_args! {
                    "to" => to,
                    "token_id" => token_id
                },
            );
        } else {
            success = runtime::call_contract(
                token_address.into_hash().unwrap_or_revert().into(),
                "safe_transfer_from",
                runtime_args! {
                    "from" => from,
                    "to" => to,
                    "token_id" => token_id
                },
            );
        }

        if success == false {
            runtime::revert(ApiError::from(Error::NftTransferFailed));
        }
    }

    // Checks if contract is nonstandard, does transferFrom according to contract implementation
    fn transfer_from_nft(&self, from: Key, to: Key, token_address: Key, token_id: U256) {
        if token_address.to_formatted_string() == data::get_kitties().to_formatted_string() {
            let (success, _): (bool, String) = runtime::call_contract(
                token_address.into_hash().unwrap_or_revert().into(),
                "transfer_from",
                runtime_args! {
                    "from" => from,
                    "to" => to,
                    "token_id" => token_id
                },
            );
            if !success {
                runtime::revert(ApiError::from(Error::TransferError));
            }
        } else if token_address.to_formatted_string() == data::get_punks().to_formatted_string() {
            let (check_success, owner): (bool, Key) = runtime::call_contract(
                token_address.into_hash().unwrap_or_revert().into(),
                "punk_index_to_address",
                runtime_args! {
                    "token_id" => token_id
                },
            );

            if !check_success
                || owner.to_formatted_string() != self.get_caller().to_formatted_string()
            {
                runtime::revert(ApiError::from(Error::InvalidOwner));
            }

            let (buy_success, _): (bool, String) = runtime::call_contract(
                token_address.into_hash().unwrap_or_revert().into(),
                "buy_punk",
                runtime_args! {
                    "token_id" => token_id
                },
            );

            if !buy_success {
                runtime::revert(ApiError::from(Error::BuyPunkResult));
            }

            let (success, _): (bool, String) = runtime::call_contract(
                token_address.into_hash().unwrap_or_revert().into(),
                "transfer_punk",
                runtime_args! {
                    "to" => to,
                    "token_id" => token_id
                },
            );
            if !success {
                runtime::revert(ApiError::from(Error::TransferError));
            }
        } else {
            let (success, _): (bool, String) = runtime::call_contract(
                token_address.into_hash().unwrap_or_revert().into(),
                "safe_transfer_from",
                runtime_args! {
                    "from" => from,
                    "to" => to,
                    "token_id" => token_id
                },
            );
            if !success {
                runtime::revert(ApiError::from(Error::TransferError));
            }
        }
    }

    fn on_erc721_received(
        &mut self,
        call_function: String,
        operator: Key,
        from: Key,
        token_id: U256,
        data: String,
    ) -> String {
        self.emit(&LiquidTransferEvent::ERC721Received {
            operator,
            from,
            token_id,
            data,
        });
        call_function
    }

    fn emit(&mut self, liquid_transfer_event: &LiquidTransferEvent) {
        let mut events = Vec::new();
        let package = data::get_contract_package_hash();
        match liquid_transfer_event {
            LiquidTransferEvent::ERC721Received {
                operator,
                from,
                token_id,
                data,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_transfer_event.type_name());
                event.insert("operator", operator.to_string());
                event.insert("from", from.to_string());
                event.insert("token_id", token_id.to_string());
                event.insert("data", data.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
