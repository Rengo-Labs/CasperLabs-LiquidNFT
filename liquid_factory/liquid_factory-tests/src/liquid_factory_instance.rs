use casper_types::{
    bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key, RuntimeArgs, URef, U256,
};
use test_env::{Sender, TestContract, TestEnv};

pub struct LIQUIDFACTORYInstance(TestContract);

impl LIQUIDFACTORYInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
        default_count: U256,
        default_token: Key,
        default_target: Key,
    ) -> LIQUIDFACTORYInstance {
        LIQUIDFACTORYInstance(TestContract::new(
            env,
            "liquid_factory.wasm",
            contract_name,
            sender,
            runtime_args! {
                "default_count" => default_count,
                "default_token" => default_token,
                "default_target" => default_target
            },
        ))
    }

    pub fn store_predictions(
        &self,
        sender: Sender,
        prediction_start: U256,
        prediction_count: U256,
        prediction_token: Key,
    ) {
        self.0.call_contract(
            sender,
            "store_predictions",
            runtime_args! {
                "prediction_start" => prediction_start,
                "prediction_count" => prediction_count,
                "prediction_token" => prediction_token,
            },
        );
    }

    pub fn predict_locker_address(
        &self,
        sender: Sender,
        index: U256,
        factory: Key,
        implementation: Key,
    ) {
        self.0.call_contract(
            sender,
            "predict_locker_address",
            runtime_args! {
                "index" => index,
                "factory" => factory,
                "implementation" => implementation
            },
        );
    }

    pub fn update_default_target(&self, sender: Sender, new_default_target: Key) {
        self.0.call_contract(
            sender,
            "update_default_target",
            runtime_args! {
                "new_default_target" => new_default_target
            },
        );
    }

    pub fn update_implementation(&self, sender: Sender, token_address: Key, target_address: Key) {
        self.0.call_contract(
            sender,
            "update_implementation",
            runtime_args! {
                "token_address" => token_address,
                "target_address" => target_address
            },
        );
    }

    pub fn update_master(&self, sender: Sender, new_master: Key) {
        self.0.call_contract(
            sender,
            "update_master",
            runtime_args! {
                "new_master" => new_master
            },
        );
    }

    pub fn revoke_master(&self, sender: Sender) {
        self.0
            .call_contract(sender, "revoke_master", runtime_args! {});
    }

    pub fn create_liquid_locker(
        &self,
        sender: Sender,
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
            "create_liquid_locker",
            runtime_args! {
                "token_id" => token_id,
                "token_address" => token_address,
                "floor_asked" => floor_asked,
                "total_asked" => total_asked,
                "payment_time" => payment_time,
                "payment_rate" => payment_rate,
                "payment_token" => payment_token
            },
        );
    }

    pub fn create_empty_locker(&self, sender: Sender, payment_token: Key) {
        self.0.call_contract(
            sender,
            "create_empty_locker",
            runtime_args! {
                "payment_token" => payment_token
            },
        );
    }

    pub fn contribute_to_locker(&self, sender: Sender, lockers_address: Key, payment_amount: U256) {
        self.0.call_contract(
            sender,
            "contribute_to_locker",
            runtime_args! {
                "lockers_address" => lockers_address,
                "payment_amount" => payment_amount
            },
        );
    }

    pub fn get_implementation(&self, sender: Sender, payment_token: Key) {
        self.0.call_contract(
            sender,
            "get_implementation",
            runtime_args! {
                "payment_token" => payment_token
            },
        );
    }

    pub fn donate_to_locker(&self, sender: Sender, lockers_address: Key, donation_amount: U256) {
        self.0.call_contract(
            sender,
            "donate_to_locker",
            runtime_args! {
                "lockers_address" => lockers_address,
                "donation_amount" => donation_amount
            },
        );
    }

    pub fn payback_to_locker(&self, sender: Sender, lockers_address: Key, payment_amount: U256) {
        self.0.call_contract(
            sender,
            "payback_to_locker",
            runtime_args! {
                "lockers_address" => lockers_address,
                "payment_amount" => payment_amount
            },
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
