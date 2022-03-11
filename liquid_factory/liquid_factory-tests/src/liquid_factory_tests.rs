use casper_engine_test_support::AccountHash;
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use renvm_sig::keccak256;
use test_env::{Sender, TestContract, TestEnv};

use crate::liquid_factory_instance::LIQUIDFACTORYInstance;

fn deploy() -> (TestEnv, AccountHash, LIQUIDFACTORYInstance) {
    let default_count: U256 = 1.into();
    let default_token: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    let default_target: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();

    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = LIQUIDFACTORYInstance::new(
        &env,
        "LIQUIDFACTORY",
        Sender(owner),
        default_count,
        default_token,
        default_target,
    );
    (env, owner, instance)
}

#[test]
fn test_deploy() {
    // let (_, _, _) = deploy();
    // unsafe {
    //     const ptr: *mut &[u8] = 0x40 as *mut _;
    //     // *ptr = "0x3d602d80600a3d3981f3363d3d373d3d3d363d73000000000000000000000000".as_bytes();
    //     println!("{:?}", ptr);
    // }

    let a = 0x99999999999999999999999999999999 as u128;

    let b = a << 0x1;
    println!("{:?}", a);
}

// #[test]
fn test_store_predictions() {
    let (_, owner, instance) = deploy();

    instance.store_predictions(
        Sender(owner),
        1.into(),
        1.into(),
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap(),
    );
}

// #[test]
fn test_predict_locker_address() {
    let (_, owner, instance) = deploy();

    instance.predict_locker_address(
        Sender(owner),
        1.into(),
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap(),
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap(),
    );
}

// #[test]
fn test_update_default_target() {
    let (_, owner, instance) = deploy();

    instance.update_default_target(
        Sender(owner),
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap(),
    );
}

// #[test]
fn test_update_implementation() {
    let (_, owner, instance) = deploy();

    instance.update_default_target(
        Sender(owner),
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap(),
    );
}

// #[test]
fn test_update_master() {
    let (_, owner, instance) = deploy();

    instance.update_master(
        Sender(owner),
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap(),
    );
}

// #[test]
fn test_revoke_master() {
    let (_, owner, instance) = deploy();

    instance.revoke_master(Sender(owner));
}

// #[test]
fn test_create_liquid_locker() {
    let (_, owner, instance) = deploy();

    let tmp: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();

    let token_id: Vec<U256> = Vec::new();
    let token_address: Key = tmp.clone();
    let floor_asked: U256 = 1.into();
    let total_asked: U256 = 1.into();
    let payment_time: U256 = 1.into();
    let payment_rate: U256 = 1.into();
    let payment_token: Key = tmp.clone();

    instance.create_liquid_locker(
        Sender(owner),
        token_id,
        token_address,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
        payment_token,
    );
}

// #[test]
fn test_create_empty_locker() {
    let (_, owner, instance) = deploy();

    let payment_token: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();

    instance.create_empty_locker(Sender(owner), payment_token);
}

// #[test]
fn test_contribute_to_locker() {
    let (_, owner, instance) = deploy();

    let lockers_address: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    let payment_amount: U256 = 1.into();

    instance.contribute_to_locker(Sender(owner), lockers_address, payment_amount);
}

// #[test]
fn test_get_implementation() {
    let (_, owner, instance) = deploy();

    let payment_token: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();

    instance.get_implementation(Sender(owner), payment_token);
}

// #[test]
fn test_donate_to_locker() {
    let (_, owner, instance) = deploy();

    let lockers_address: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    let donation_amount: U256 = 1.into();

    instance.donate_to_locker(Sender(owner), lockers_address, donation_amount);
}

// #[test]
fn test_payback_to_locker() {
    let (_, owner, instance) = deploy();

    let lockers_address: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    let payment_amount: U256 = 1.into();

    instance.payback_to_locker(Sender(owner), lockers_address, payment_amount);
}
