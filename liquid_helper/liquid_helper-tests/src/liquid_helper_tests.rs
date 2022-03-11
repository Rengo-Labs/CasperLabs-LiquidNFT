use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512};
use renvm_sig::keccak256;
use test_env::{Sender, TestContract, TestEnv};

use crate::liquid_helper_instance::LIQUIDHELPERInstance;

fn deploy() -> (TestEnv, AccountHash, LIQUIDHELPERInstance) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = LIQUIDHELPERInstance::new(&env, "LIQUIDHELPER", Sender(owner));
    (env, owner, instance)
}

// #[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}

#[test]
fn test_kyc() {
    let (_, owner, instance) = deploy();
    let mut credentials: Vec<String> = Vec::new();

    // Demo KYC Credentials
    credentials.push("35202-4177167-9".to_string());
    credentials.push("Bassam Monib".to_string());
    credentials.push("0341-4230502".to_string());

    let mut packed: String = "".to_string();
    for i in &credentials {
        packed.push_str(&hex::encode(i));
    }
    let res: [u8; 32] = keccak256(&packed.as_bytes());

    instance.liquid_helper(Sender(owner), credentials);
    let ret: [u8; 32] = instance.result();

    assert_eq!(res, ret);
}
