use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash,
    RuntimeArgs,
};
use test_env::{TestContract, TestEnv};

pub struct LIQUIDHELPERInstance(TestContract);

impl LIQUIDHELPERInstance {
    pub fn new(env: &TestEnv, contract_name: &str, sender: AccountHash) -> LIQUIDHELPERInstance {
        LIQUIDHELPERInstance(TestContract::new(
            env,
            "liquid-helper.wasm",
            contract_name,
            sender,
            runtime_args! {},
            0,
        ))
    }

    pub fn liquid_helper(&self, sender: AccountHash, credentials: Vec<String>) {
        self.0.call_contract(
            sender,
            "liquid_helper",
            runtime_args! {
                "credentials" => credentials
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
