use std::collections::BTreeMap;

use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use test_env::{TestContract, TestEnv};

use crate::liquid_factory_instance::{now, LIQUIDFACTORYInstance};

const ONE_MINUTE_IN_MS: u64 = 60000;

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
        now(),
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
        now(),
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
        now(),
    );

    (env, owner, instance, erc20)
}

fn init() -> (
    TestEnv,
    AccountHash,
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
    let total_asked: U256 = 2_000_000_000.into();
    let payment_time: U256 = 86400000.into();
    let payment_rate: U256 = 100_000.into();
    let payment_token: Key = Key::Hash(erc20.package_hash());

    let token_ids: Vec<U256> = vec![1.into(), 2.into()];

    let mut token_metas: Vec<BTreeMap<String, String>> = Vec::default();

    let mut meta: BTreeMap<String, String> = BTreeMap::default();
    meta.insert("TOKEN-1".into(), "Metadata for token1".into());
    token_metas.push(meta);

    let mut meta: BTreeMap<String, String> = BTreeMap::default();
    meta.insert("TOKEN-2".into(), "Metadata for token2".into());
    token_metas.push(meta);

    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => U256::from(1_000_000_000_000u64)
        },
        now(),
    );

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

    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(factory_instance.package_hash()),
            "amount" => U256::from(1_000_000_000_000u64),
        },
        now(),
    );

    cep47.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(factory_instance.package_hash()),
            "token_ids" => token_ids.clone(),
        },
        now(),
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
        now(),
    );

    let (lockers_contract_address, lockers_package_address): (Key, Key) =
        factory_instance.query("result");

    (
        env,
        owner,
        factory_instance,
        erc20,
        cep47,
        lockers_contract_address,
        lockers_package_address,
    )
}

#[test]
#[should_panic]
fn should_not_be_able_to_contribute_after_contribution_phase_end() {
    let (
        _env,
        owner,
        factory_instance,
        _erc20,
        _cep47,
        _lockers_contract_address,
        lockers_package_address,
    ) = init();
    let payment_amount: U256 = 100_000.into();

    factory_instance.contribute_to_locker(
        owner,
        lockers_package_address,
        payment_amount,
        now() + (ONE_MINUTE_IN_MS * 500_000_000),
    );
}
