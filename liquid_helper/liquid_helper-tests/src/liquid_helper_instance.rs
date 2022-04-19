use casper_types::{
    bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key, RuntimeArgs, URef, U256,
};
use test_env::{Sender, TestContract, TestEnv};

pub struct LIQUIDHELPERInstance(TestContract);

impl LIQUIDHELPERInstance {
    pub fn new(env: &TestEnv, contract_name: &str, sender: Sender) -> LIQUIDHELPERInstance {
        LIQUIDHELPERInstance(TestContract::new(
            env,
            "liquid_helper.wasm",
            contract_name,
            sender,
            runtime_args! {},
        ))
    }

    pub fn liquid_helper(&self, sender: Sender, credentials: Vec<String>) {
        self.0.call_contract(
            sender,
            "liquid_helper",
            runtime_args! {
                "credentials" => credentials
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
