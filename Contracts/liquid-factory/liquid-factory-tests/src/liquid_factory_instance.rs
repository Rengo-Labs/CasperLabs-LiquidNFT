use std::time::SystemTime;

use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub struct LIQUIDFACTORYInstance(TestContract);

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

impl LIQUIDFACTORYInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        default_count: U256,
        default_token: Key,
        default_target: Key,
        time: u64,
    ) -> LIQUIDFACTORYInstance {
        LIQUIDFACTORYInstance(TestContract::new(
            env,
            "liquid-factory.wasm",
            contract_name,
            sender,
            runtime_args! {
                "default_count" => default_count,
                "default_token" => default_token,
                "default_target" => default_target
            },
            time,
        ))
    }

    pub fn update_master(&self, sender: AccountHash, new_master: Key) {
        self.0.call_contract(
            sender,
            "update_master",
            runtime_args! {
                "new_master" => new_master
            },
            0,
        );
    }

    pub fn revoke_master(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "revoke_master", runtime_args! {}, 0);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_liquid_locker(
        &self,
        sender: AccountHash,
        token_id: Vec<U256>,
        token_address: Key,
        floor_asked: U256,
        total_asked: U256,
        payment_time: U256,
        payment_rate: U256,
        payment_token: Key,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "create_liquid_locker_js_client",
            runtime_args! {
                "token_id" => token_id,
                "token_address" => token_address,
                "floor_asked" => floor_asked,
                "total_asked" => total_asked,
                "payment_time" => payment_time,
                "payment_rate" => payment_rate,
                "payment_token" => payment_token
            },
            time,
        );
    }

    pub fn create_empty_locker(&self, sender: AccountHash, payment_token: Key, time: u64) {
        self.0.call_contract(
            sender,
            "create_empty_locker_js_client",
            runtime_args! {
                "payment_token" => payment_token
            },
            time,
        );
    }

    pub fn contribute_to_locker(
        &self,
        sender: AccountHash,
        lockers_address: Key,
        payment_amount: U256,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "contribute_to_locker_js_client",
            runtime_args! {
                "lockers_address" => lockers_address,
                "payment_amount" => payment_amount
            },
            time,
        );
    }

    pub fn donate_to_locker(
        &self,
        sender: AccountHash,
        lockers_address: Key,
        donation_amount: U256,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "donate_to_locker",
            runtime_args! {
                "lockers_address" => lockers_address,
                "donation_amount" => donation_amount
            },
            time,
        );
    }

    pub fn payback_to_locker(
        &self,
        sender: AccountHash,
        lockers_address: Key,
        payment_amount: U256,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "payback_to_locker",
            runtime_args! {
                "lockers_address" => lockers_address,
                "payment_amount" => payment_amount
            },
            time,
        );
    }

    // Result methods
    pub fn query<T: CLTyped + FromBytes>(&self, key: &str) -> T {
        self.0.query_named_key(key.into())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }
}
