use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub struct LIQUIDTRANSFERInstance(TestContract);

impl LIQUIDTRANSFERInstance {
    pub fn new(env: &TestEnv, contract_name: &str, sender: AccountHash) -> LIQUIDTRANSFERInstance {
        LIQUIDTRANSFERInstance(TestContract::new(
            env,
            "liquid_locker.wasm",
            contract_name,
            sender,
            runtime_args! {},
            0,
        ))
    }
}
