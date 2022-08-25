use std::collections::BTreeMap;

use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use test_env::{call_contract_with_hash, TestContract, TestEnv};

use crate::liquid_factory_instance::LIQUIDFACTORYInstance;

const DAYS_IN_MILLI_SEC: u64 = 86400000;
const TIME: u64 = 400_000_000;

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
    let payment_time: U256 = 86400000.into();
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
        0,
    );

    let (lockers_contract_address, lockers_package_address): (Key, Key) =
        factory_instance.query("result");

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

#[test]
fn test_contract_flow() {
    let (
        env,
        accounts,
        factory_instance,
        erc20,
        _cep47,
        lockers_contract_address,
        lockers_package_address,
    ) = init();

    factory_instance.contribute_to_locker(
        accounts[0],
        lockers_package_address,
        3_000_000_000u64.into(),
        TIME,
    );
    factory_instance.contribute_to_locker(
        accounts[1],
        lockers_package_address,
        1_000_000_000u64.into(),
        TIME,
    );

    call_contract_with_hash(
        &env,
        lockers_contract_address.into_hash().unwrap().into(),
        accounts[0],
        "enable_locker",
        runtime_args! {
            "prepay_amount" => U256::from(1_200_000)
        },
        DAYS_IN_MILLI_SEC * 5,
    );

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        DAYS_IN_MILLI_SEC * 5,
    );
    let old_balance: U256 = erc20.query_named_key("balance".into());
    assert_eq!(old_balance, 1_000_198_800_000u64.into());

    let payment_amount: U256 = 5_000_000_000u64.into();
    factory_instance.payback_to_locker(
        accounts[0],
        lockers_package_address,
        payment_amount,
        DAYS_IN_MILLI_SEC * 5,
    );

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        DAYS_IN_MILLI_SEC * 5,
    );
    let new_balance: U256 = erc20.query_named_key("balance".into());
    assert_eq!(
        new_balance,
        old_balance - payment_amount,
        "Payback not done"
    );

    let donation_amount: U256 = 50.into();
    factory_instance.donate_to_locker(accounts[0], lockers_package_address, donation_amount);

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        0,
    );
    assert_eq!(
        new_balance - donation_amount,
        erc20.query_named_key("balance".into()),
        "Doantion not performed"
    );
}

#[test]
fn should_be_able_to_contribute_before_contribution_phase_end() {
    const TIME: u64 = 400_000;
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
        TIME,
    );
    let (total_increase, users_increase): (U256, U256) = factory_instance.query("result");
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
fn should_return_back_nft_to_owner_as_floor_not_reached_in_contribution_phase() {
    let (
        env,
        accounts,
        factory_instance,
        _erc20,
        cep47,
        lockers_contract_address,
        lockers_package_address,
    ) = init();

    let payment_amount: U256 = 100_000.into();
    factory_instance.contribute_to_locker(
        accounts[0],
        lockers_package_address,
        payment_amount,
        TIME,
    );

    let (total_increase, users_increase): (U256, U256) = factory_instance.query("result");
    assert_eq!(
        total_increase, payment_amount,
        "Total contribution not increased"
    );
    assert_eq!(
        users_increase, payment_amount,
        "User contribution not increased"
    );

    cep47.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        TIME,
    );
    let balance: U256 = cep47.query_named_key("balance".into());
    assert_eq!(
        balance,
        0.into(),
        "Should be zero as both 2 NFT's given to factory"
    );

    call_contract_with_hash(
        &env,
        lockers_contract_address.into_hash().unwrap().into(),
        accounts[0],
        "disable_locker",
        runtime_args! {},
        TIME,
    );

    cep47.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        TIME,
    );
    let balance: U256 = cep47.query_named_key("balance".into());
    assert_eq!(balance, 2.into(), "2 NFT's not returned to owner");
}

#[test]
fn should_be_able_to_liquadate_locker_as_owner_didnt_do_payment_for_days() {
    const TIME: u64 = 400_000;
    let (
        env,
        accounts,
        factory_instance,
        _erc20,
        cep47,
        lockers_contract_address,
        lockers_package_address,
    ) = init();
    let payment_amount: U256 = 5_000_000_000u64.into();
    factory_instance.contribute_to_locker(
        accounts[1],
        lockers_package_address,
        payment_amount,
        TIME,
    );
    let (total_increase, users_increase): (U256, U256) = factory_instance.query("result");
    assert_eq!(
        total_increase, payment_amount,
        "Total contribution not increased"
    );
    assert_eq!(
        users_increase, payment_amount,
        "User contribution not increased"
    );

    cep47.call_contract(
        accounts[1],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[1]) },
        TIME,
    );
    let balance: U256 = cep47.query_named_key("balance".into());
    assert_eq!(balance, 0.into(), "Should not have NFT initially");

    call_contract_with_hash(
        &env,
        lockers_contract_address.into_hash().unwrap().into(),
        accounts[0],
        "liquidate_locker",
        runtime_args! {},
        TIME + (DAYS_IN_MILLI_SEC * 15),
    );

    cep47.call_contract(
        accounts[1],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[1]) },
        TIME,
    );
    let balance: U256 = cep47.query_named_key("balance".into());
    assert_eq!(balance, 2.into(), "2 NFT's not liquidated to contributor");
}