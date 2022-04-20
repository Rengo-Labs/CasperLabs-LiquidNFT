use crate::liquid_helper_instance::LIQUIDHELPERInstance;
use casper_types::{account::AccountHash, Key, U256};
use test_env::{TestContract, TestEnv};

fn deploy() -> (TestEnv, AccountHash, TestContract, LIQUIDHELPERInstance) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let contract = LIQUIDHELPERInstance::new(&env, "LIQUIDHELPER", owner);
    let proxy = LIQUIDHELPERInstance::proxy(
        &env,
        "LIQUIDHELPERPROXY",
        owner,
        Key::Hash(contract.package_hash()),
    );
    (env, owner, contract, proxy)
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}

#[test]
fn get_tokens() {
    let (_, owner, _, proxy) = deploy();

    proxy.get_tokens(owner);
    let res: Vec<U256> = proxy.result();
    let mut compare: Vec<U256> = Vec::new();
    assert_eq!(compare, res);
}

#[test]
fn ownerless_locker() {
    let (_, owner, _, proxy) = deploy();

    proxy.ownerless_locker(owner);
    let res: bool = proxy.result();
    assert_eq!(true, res);
}

#[test]
fn floor_not_reached() {
    let (_, owner, _, proxy) = deploy();

    proxy.floor_not_reached(owner);
    let res: bool = proxy.result();
    assert_eq!(false, res);
}

#[test]
fn not_single_provider() {
    let (_, owner, _, proxy) = deploy();
    let tmp_check_address: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    proxy.not_single_provider(owner, tmp_check_address);

    let res: bool = proxy.result();

    assert_eq!(false, res);
}

#[test]
fn reached_total() {
    let (_, owner, _, proxy) = deploy();
    let arg_token_amount: U256 = 1.into();
    let arg_contributor: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    proxy.reached_total(owner, arg_contributor, arg_token_amount);

    let res: bool = proxy.result();
    assert_eq!(true, res);
}

#[test]
fn missed_activate() {
    let (_, owner, _, proxy) = deploy();
    proxy.missed_activate(owner);
    let res: bool = proxy.result();
    assert_eq!(false, res);
}

#[test]
fn missed_deadline() {
    let (_, owner, _, proxy) = deploy();
    proxy.missed_deadline(owner);
    let res: bool = proxy.result();
    assert_eq!(false, res);
}

#[test]
fn below_floor_asked() {
    let (_, owner, _, proxy) = deploy();
    proxy.below_floor_asked(owner);
    let res: bool = proxy.result();

    assert_eq!(false, res);
}

#[test]
fn payment_time_not_set() {
    let (_, owner, _, proxy) = deploy();
    proxy.payment_time_not_set(owner);
    let res: bool = proxy.result();
    assert_eq!(true, res);
}

#[test]
fn contribution_phase() {
    let (_, owner, _, proxy) = deploy();
    proxy.contribution_phase(owner);
    let res: bool = proxy.result();

    assert_eq!(true, res);
}

#[test]
fn payback_timestamp() {
    let (_, owner, _, proxy) = deploy();
    proxy.payback_timestamp(owner);
    let compare: U256 = (432000).into();
    let res: U256 = proxy.result();
    assert_eq!(compare, res);
}

#[test]
fn liquidate_to() {
    let (_, owner, _, proxy) = deploy();
    proxy.liquidate_to(owner);
    let res_key: Key = proxy.result();
    let compare_key: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    assert_eq!(compare_key, res_key);
}

#[test]
fn time_since() {
    let (_, owner, _, proxy) = deploy();
    const TIME: u64 = 10000;

    let param_time_stamp: U256 = 1.into();
    proxy.time_since(owner, param_time_stamp, TIME);

    let res: U256 = proxy.result();
    assert_eq!(U256::from(9999), res);
}

#[test]
fn starting_timestamp() {
    let (_, owner, _, proxy) = deploy();
    proxy.starting_timestamp(owner);
    let res: U256 = proxy.result();
    let compare: U256 = (432000).into();

    assert_eq!(compare, res);
}
