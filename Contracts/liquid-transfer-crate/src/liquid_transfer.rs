use alloc::vec::Vec;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use contract_utils::{ContractContext, ContractStorage};

pub trait LIQUIDTRANSFER<Storage: ContractStorage>: ContractContext<Storage> {
    fn transfer_nft(&self, token: Key, recipient: Key, token_ids: Vec<U256>) {
        let () = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "token_ids" => token_ids
            },
        );
    }

    fn transfer_from_nft(&self, sender: Key, recipient: Key, token: Key, token_ids: Vec<U256>) {
        let () = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "transfer_from",
            runtime_args! {
                "sender" => sender,
                "recipient" => recipient,
                "token_ids" => token_ids
            },
        );
    }
}
