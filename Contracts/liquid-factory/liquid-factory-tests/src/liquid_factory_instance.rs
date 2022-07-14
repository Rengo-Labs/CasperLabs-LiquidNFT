use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub struct LIQUIDFACTORYInstance(TestContract);

impl LIQUIDFACTORYInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        default_count: U256,
        default_token: Key,
        default_target: Key,
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
            0,
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
            0,
        );
    }

    pub fn create_empty_locker(&self, sender: AccountHash, payment_token: Key) {
        self.0.call_contract(
            sender,
            "create_empty_locker_js_client",
            runtime_args! {
                "payment_token" => payment_token
            },
            0,
        );
    }

    pub fn contribute_to_locker(
        &self,
        sender: AccountHash,
        lockers_address: Key,
        payment_amount: U256,
    ) {
        self.0.call_contract(
            sender,
            "contribute_to_locker_js_client",
            runtime_args! {
                "lockers_address" => lockers_address,
                "payment_amount" => payment_amount
            },
            0,
        );
    }

    pub fn donate_to_locker(
        &self,
        sender: AccountHash,
        lockers_address: Key,
        donation_amount: U256,
    ) {
        self.0.call_contract(
            sender,
            "donate_to_locker",
            runtime_args! {
                "lockers_address" => lockers_address,
                "donation_amount" => donation_amount
            },
            0,
        );
    }

    pub fn payback_to_locker(
        &self,
        sender: AccountHash,
        lockers_address: Key,
        payment_amount: U256,
    ) {
        self.0.call_contract(
            sender,
            "payback_to_locker",
            runtime_args! {
                "lockers_address" => lockers_address,
                "payment_amount" => payment_amount
            },
            0,
        );
    }

    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }
}
