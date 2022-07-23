use std::collections::BTreeMap;

use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use test_env::{call_contract_with_hash, TestContract, TestEnv};

use crate::liquid_factory_instance::LIQUIDFACTORYInstance;

fn zero_address() -> Key {
    Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000")
        .unwrap()
}

fn deploy_cep47(env: &TestEnv, owner: AccountHash, meta: BTreeMap<String, String>) -> TestContract {
    TestContract::new(
        env,
        "cep47-token.wasm",
        "cep47",
        owner,
        runtime_args! {
            "name" => "CEP",
            "symbol" => "CEP-47",
            "meta" => meta
        },
        0,
    )
}

fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "erc20",
        owner,
        runtime_args! {
            "name" => "ERC",
            "symbol" => "ERC20",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(0)
        },
        0,
    )
}

fn deploy() -> (TestEnv, AccountHash, LIQUIDFACTORYInstance, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);

    let default_count: U256 = 0.into();
    let default_token: Key = Key::Hash(erc20.package_hash());
    let default_target: Key = zero_address();

    let instance = LIQUIDFACTORYInstance::new(
        &env,
        "LIQUIDFACTORY",
        owner,
        default_count,
        default_token,
        default_target,
    );

    (env, owner, instance, erc20)
}

fn init() -> (
    TestEnv,
    Vec<AccountHash>,
    LIQUIDFACTORYInstance,
    TestContract,
    TestContract,
    Key,
    Key,
) {
    let (env, owner, factory_instance, erc20) = deploy();

    let cep47 = deploy_cep47(&env, owner, BTreeMap::default());

    let token_address: Key = Key::Hash(cep47.package_hash());
    let floor_asked: U256 = 1_000_000_000.into();
    let total_asked: U256 = 5_000_000_000u64.into();
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

    let accounts: Vec<AccountHash> = vec![owner, env.next_user(), env.next_user()];

    for account in &accounts {
        erc20.call_contract(
            *account,
            "mint",
            runtime_args! {
                "to" => Key::Account(*account),
                "amount" => U256::from(1_000_000_000_000u64)
            },
            0,
        );

        erc20.call_contract(
            *account,
            "approve",
            runtime_args! {
                "spender" => Key::from(factory_instance.package_hash()),
                "amount" => U256::from(1_000_000_000_000u64),
            },
            0,
        );
    }

    cep47.call_contract(
        owner,
        "mint",
        runtime_args! {
            "recipient" => Key::Account(owner),
            "token_ids" => token_ids.clone(),
            "token_metas" => token_metas,
        },
        0,
    );

    cep47.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(factory_instance.package_hash()),
            "token_ids" => token_ids.clone(),
        },
        0,
    );

    factory_instance.create_liquid_locker(
        owner,
        token_ids,
        token_address,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
        payment_token,
    );

    let (lockers_contract_address, lockers_package_address): (Key, Key) = factory_instance.result();

    (
        env,
        accounts,
        factory_instance,
        erc20,
        cep47,
        lockers_contract_address,
        lockers_package_address,
    )
}

/// Contribution phase limit == 5 days (432000 seconds)

#[test]
fn should_be_able_to_contribute_before_contribution_phase_end() {
    let (
        _env,
        accounts,
        factory_instance,
        _erc20,
        _cep47,
        _lockers_contract_address,
        lockers_package_address,
    ) = init();
    let payment_amount: U256 = 100_000.into();
    factory_instance.contribute_to_locker(
        accounts[0],
        lockers_package_address,
        payment_amount,
        400_000,
    );
    let (total_increase, users_increase): (U256, U256) = factory_instance.result();
    assert_eq!(
        total_increase, payment_amount,
        "Total contribution not increased"
    );
    assert_eq!(
        users_increase, payment_amount,
        "User contribution not increased"
    );
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}

#[test]
fn test_update_master() {
    let (_, owner, instance, _) = deploy();

    instance.update_master(owner, Key::Account(owner));
}

#[test]
fn test_revoke_master() {
    let (_, owner, instance, _) = deploy();

    instance.revoke_master(owner);
}

#[test]
fn test_create_liquid_locker() {
    let (env, owner, instance, erc20) = deploy();

    let cep47 = deploy_cep47(&env, owner, BTreeMap::default());

    let token_address: Key = Key::Hash(cep47.package_hash());
    let floor_asked: U256 = 1.into();
    let total_asked: U256 = 1.into();
    let payment_time: U256 = 1.into();
    let payment_rate: U256 = 1.into();
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
        0,
    );

    cep47.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(instance.package_hash()),
            "token_ids" => token_ids.clone(),
        },
        0,
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
    );
}

#[test]
fn test_create_empty_locker() {
    let (_, owner, instance, erc20) = deploy();

    let payment_token: Key = Key::Hash(erc20.package_hash());

    instance.create_empty_locker(owner, payment_token);
}

#[test]
fn test_contribute_to_locker() {
    let (_, owner, instance, erc20) = deploy();

    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => U256::from(1000)
        },
        0,
    );

    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(instance.package_hash()),
            "amount" => U256::from(1000)
        },
        0,
    );

    let payment_token: Key = Key::Hash(erc20.package_hash());
    let payment_amount: U256 = 10.into();

    instance.create_empty_locker(owner, payment_token);
    let (_, lockers_package_address): (Key, Key) = instance.result();

    instance.contribute_to_locker(owner, lockers_package_address, payment_amount, 0);
}

#[test]
fn test_donate_to_locker() {
    let (_, owner, instance, erc20) = deploy();

    let payment_token: Key = Key::Hash(erc20.package_hash());
    instance.create_empty_locker(owner, payment_token);
    let (_, lockers_package_address): (Key, Key) = instance.result();
    let donation_amount: U256 = 1.into();

    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => U256::from(1000)
        },
        0,
    );

    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(instance.package_hash()),
            "amount" => U256::from(1000)
        },
        0,
    );

    instance.donate_to_locker(owner, lockers_package_address, donation_amount);
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
        3_000_000_000u64.into(),
        400_000,
    );
    factory_instance.contribute_to_locker(
        accounts[1],
        lockers_package_address,
        1_000_000_000u64.into(),
        400_000,
    );

    call_contract_with_hash(
        &env,
        lockers_contract_address.into_hash().unwrap().into(),
        accounts[0],
        "enable_locker",
        runtime_args! {
            "prepay_amount" => U256::from(1_200)
        },
        86400 * 5,
    );

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        86400 * 5,
    );
    let old_balance: U256 = erc20.query_named_key("balance".into());
    assert_eq!(old_balance, 1_000_199_998_800u64.into());

    factory_instance.payback_to_locker(
        accounts[0],
        lockers_package_address,
        5_000_000_000u64.into(),
        86400 * 5,
    );

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        86400 * 5,
    );
    let new_nalance: U256 = erc20.query_named_key("balance".into());
    assert_eq!(
        new_nalance,
        (1_000_199_998_800u64 - 5_000_000_000u64).into(),
        "Payback not done"
    );
}
