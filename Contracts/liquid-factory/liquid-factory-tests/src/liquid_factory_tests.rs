use crate::liquid_factory_instance::*;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use std::collections::BTreeMap;
use test_env::call_contract_with_hash;

#[test]
fn test_deploy() {
    let (
        _env,
        _accounts,
        _factory_instance,
        _erc20,
        _,
        _lockers_contract_address,
        _lockers_package_address,
    ) = init();
}

#[test]
fn test_update_master() {
    let (env, owner, instance, _) = deploy();
    assert_eq!(
        Key::Account(owner),
        instance.query("master_address"),
        "Owner is not master"
    );
    let new_master: Key = Key::Account(env.next_user());
    instance.update_master(owner, new_master);
    assert_eq!(
        new_master,
        instance.query("master_address"),
        "Master not updated"
    );
}

#[test]
fn test_revoke_master() {
    let (_, owner, instance, _) = deploy();
    assert_eq!(
        Key::Account(owner),
        instance.query("master_address"),
        "Owner is not master"
    );
    instance.revoke_master(owner);
    assert_eq!(
        account_zero_address(),
        instance.query("master_address"),
        "Master not revoked"
    );
}

#[test]
fn test_create_liquid_locker() {
    let (env, owner, instance, erc20) = deploy();

    let cep47 = deploy_cep47(&env, owner, BTreeMap::default());

    let token_address: Key = Key::Hash(cep47.package_hash());
    let floor_asked: U256 = 1_000_000_000.into();
    let total_asked: U256 = 10_000_000_000u64.into();
    let payment_time: U256 = 86400.into();
    let payment_rate: U256 = 100.into();
    let payment_token: Key = Key::Hash(erc20.package_hash());

    let token_ids: Vec<U256> = vec![1.into(), 2.into()];

    let mut token_metas: Vec<BTreeMap<String, String>> = Vec::default();

    let mut meta: BTreeMap<String, String> = BTreeMap::default();
    meta.insert("TOKEN-1".into(), "Metadata for token1".into());
    token_metas.push(meta);

    let mut meta: BTreeMap<String, String> = BTreeMap::default();
    meta.insert("TOKEN-2".into(), "Metadata for token2".into());
    token_metas.push(meta);

    cep47.call_contract(
        owner,
        "mint",
        runtime_args! {
            "recipient" => Key::Account(owner),
            "token_ids" => token_ids.clone(),
            "token_metas" => token_metas,
        },
        now(),
    );

    cep47.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(instance.package_hash()),
            "token_ids" => token_ids.clone(),
        },
        now(),
    );

    instance.create_liquid_locker(
        owner,
        token_ids,
        token_address,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
        payment_token,
        now(),
    );

    let (lockers_contract_address, lockers_package_address): (Key, Key) = instance.query("result");

    assert_ne!(
        lockers_contract_address,
        zero_address(),
        "Locker contract hash not generated"
    );
    assert_ne!(
        lockers_package_address,
        zero_address(),
        "Locker contract package hash not generated"
    );
}

#[test]
fn test_contribute_to_locker() {
    let (
        _env,
        accounts,
        factory_instance,
        erc20,
        _,
        _lockers_contract_address,
        lockers_package_address,
    ) = init();

    factory_instance.contribute_to_locker(
        accounts[0],
        lockers_package_address,
        3_000_000_000u64.into(),
        now() + (ONE_MINUTE_IN_MS * 5),
    );

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        now() + (ONE_MINUTE_IN_MS * 10),
    );
    let old_balance: U256 = erc20.query_named_key("balance".into());
    assert_eq!(old_balance, 9987000000000u64.into());
}

#[test]
fn test_donate_to_locker() {
    let (_, accounts, factory_instance, erc20, _, _, lockers_package_address) = init();

    let donation_amount: U256 = 10.into();

    erc20.call_contract(
        accounts[0],
        "mint",
        runtime_args! {
            "to" => Key::Account(accounts[0]),
            "amount" => U256::from(1000)
        },
        now(),
    );

    erc20.call_contract(
        accounts[0],
        "approve",
        runtime_args! {
            "spender" => Key::from(factory_instance.package_hash()),
            "amount" => U256::from(1000)
        },
        now(),
    );

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        now(),
    );

    assert_eq!(
        U256::from(9990000001000u64),
        erc20.query_named_key("balance".into())
    );
    factory_instance.donate_to_locker(accounts[0], lockers_package_address, donation_amount, now());

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        now(),
    );
    assert_eq!(
        U256::from(9990000001000u64) - donation_amount,
        erc20.query_named_key("balance".into()),
        "Doantion not performed"
    );
}

#[test]
fn test_payback_to_locker() {
    let (
        env,
        accounts,
        factory_instance,
        erc20,
        _,
        lockers_contract_address,
        lockers_package_address,
    ) = init();

    factory_instance.contribute_to_locker(
        accounts[0],
        lockers_package_address,
        200_000_000_000u64.into(),
        now(),
    );
    factory_instance.contribute_to_locker(
        accounts[1],
        lockers_package_address,
        400_000_000_000u64.into(),
        now(),
    );

    call_contract_with_hash(
        &env,
        lockers_contract_address.into_hash().unwrap().into(),
        accounts[0],
        "enable_locker",
        runtime_args! {
            "prepay_amount" => U256::from(200_000_000)
        },
        now() + (ONE_DAY_IN_MS * 5),
    );

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        now() + (ONE_MINUTE_IN_MS * 20),
    );
    let old_balance: U256 = erc20.query_named_key("balance".into());
    assert_eq!(old_balance, 10_389_800_000_000u64.into());

    factory_instance.payback_to_locker(
        accounts[0],
        lockers_package_address,
        200_000_000u64.into(),
        now() + (ONE_DAY_IN_MS * 10),
    );

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        now() + (ONE_DAY_IN_MS * 10),
    );
    let new_balance: U256 = erc20.query_named_key("balance".into());
    assert_eq!(
        new_balance,
        (10_389_800_000_000u64 - 200_000_000u64).into(),
        "Payback not done"
    );
}
