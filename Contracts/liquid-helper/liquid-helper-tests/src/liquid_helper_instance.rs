use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub struct LIQUIDHELPERInstance(TestContract);

impl LIQUIDHELPERInstance {
    pub fn contract_instance(contract: TestContract) -> LIQUIDHELPERInstance {
        LIQUIDHELPERInstance(contract)
    }
    pub fn new(env: &TestEnv, contract_name: &str, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "liquid-helper.wasm",
            contract_name,
            sender,
            runtime_args! {},
            0,
        )
    }
    pub fn proxy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        liquid_helper: Key,
    ) -> LIQUIDHELPERInstance {
        LIQUIDHELPERInstance(TestContract::new(
            env,
            "liquid-helper-proxy.wasm",
            contract_name,
            sender,
            runtime_args! {"liquid_helper"=>liquid_helper},
            0,
        ))
    }
    pub fn get_tokens(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "get_tokens", runtime_args! {}, 0);
    }
    pub fn floor_not_reached(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "floor_not_reached", runtime_args! {}, 0);
    }
    pub fn ownerless_locker(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "ownerless_locker", runtime_args! {}, 0);
    }
    pub fn not_single_provider(&self, sender: AccountHash, check_address: Key) {
        self.0.call_contract(
            sender,
            "not_single_provider",
            runtime_args! {
                "check_address"=> check_address

            },
            0,
        );
    }
    pub fn below_floor_asked(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "below_floor_asked", runtime_args! {}, 0);
    }
    pub fn payment_time_not_set(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "payment_time_not_set", runtime_args! {}, 0);
    }
    pub fn contribution_phase(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "contribution_phase", runtime_args! {}, 0);
    }
    pub fn payback_timestamp(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "payback_timestamp", runtime_args! {}, 0);
    }
    pub fn liquidate_to(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "liquidate_to", runtime_args! {}, 0);
    }
    pub fn time_since(&self, sender: AccountHash, time_stamp: U256, time: u64) {
        self.0.call_contract(
            sender,
            "time_since",
            runtime_args! {
                "time_stamp"=> time_stamp
            },
            time,
        );
    }
    pub fn reached_total(&self, sender: AccountHash, contributor: Key, token_amount: U256) {
        self.0.call_contract(
            sender,
            "reached_total",
            runtime_args! {

            "contributor"=>contributor,
            "token_amount"=> token_amount
            },
            0,
        );
    }
    pub fn missed_activate(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "missed_activate", runtime_args! {}, 0);
    }
    pub fn missed_deadline(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "missed_deadline", runtime_args! {}, 0);
    }
    pub fn starting_timestamp(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "starting_timestamp", runtime_args! {}, 0);
    }

    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }
}
