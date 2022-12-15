use std::collections::BTreeMap;

use crate::liquid_locker_instance::LIQUIDLOCKERInstance;
use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

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
        "erc2020",
        owner,
        runtime_args! {
            "name" => "ERC",
            "symbol" => "ERC20",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(1000000000000000_u128)
        },
        0,
    )
}

fn deploy() -> (TestEnv, AccountHash, TestContract, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);
    let proxy = LIQUIDLOCKERInstance::proxy(
        &env,
        "LIQUIDLOCKERPROXY",
        owner,
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
    );
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let contract = LIQUIDLOCKERInstance::new(
        &env,
        "LIQUIDLOCKER",
        owner,
        Key::from(package_hash),
        Key::Hash(erc20.package_hash()),
    );
    proxy.call_contract(
        owner,
        "set_liquid_locker",
        runtime_args! {"token" => Key::Hash(contract.package_hash())},
        0,
    );
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {"to" =>  Key::Hash(contract.package_hash()),"amount"=> U256::from(2146000000000u64)},
        0,
    );
    (env, owner, contract, proxy)
}

#[allow(clippy::too_many_arguments)]
fn initialize(
    env: &TestEnv,
    owner: AccountHash,
    instance: &LIQUIDLOCKERInstance,
    token_owner: Key,
    floor_asked: U256,
    total_asked: U256,
    payment_time: U256,
    payment_rate: U256,
) {
    let cep47 = deploy_cep47(env, owner, BTreeMap::default());

    let token_address: Key = Key::Hash(cep47.package_hash());

    let mut token_ids: Vec<U256> = Vec::new();
    let mut token_metas: Vec<BTreeMap<String, String>> = Vec::default();

    token_ids.push(1.into());
    token_ids.push(2.into());

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
            "recipient" => token_owner,
            "token_ids" => token_ids.clone(),
            "token_metas" => token_metas,
        },
        0,
    );

    instance.initialize(
        owner,
        token_ids,
        token_address,
        token_owner,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
    );
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}

#[test]
fn test_intialize() {
    let (env, owner, contract, _) = deploy();
    initialize(
        &env,
        owner,
        &LIQUIDLOCKERInstance::contract_instance(contract),
        Key::Account(owner),
        1.into(),
        1.into(),
        1000.into(),
        100.into(),
    )
}

#[test]
fn test_increase_payment_rate() {
    let (env, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let token_owner: Key = Key::from(package_hash);
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    initialize(
        &env,
        owner,
        &proxy,
        token_owner,
        1.into(),
        1.into(),
        1000.into(),
        1000.into(),
    );
    let new_payment_rate: U256 = U256::from(1000000000);
    proxy.increase_payment_rate(owner, new_payment_rate);
}

#[test]
fn test_decrease_payment_time() {
    let (env, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let token_owner: Key = Key::from(package_hash);
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    initialize(
        &env,
        owner,
        &proxy,
        token_owner,
        1.into(),
        1.into(),
        1000.into(),
        1000.into(),
    );
    let new_payment_time: U256 = U256::from(0);
    proxy.decrease_payment_time(owner, new_payment_time);
}

#[test]
fn test_contribution_phase() {
    let (_, owner, _, proxy) = deploy();
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    proxy.make_contribution(owner, 1000.into(), Key::Account(owner));
}
#[test]
fn test_calculate_epoch() {
    let (_, owner, _, proxy) = deploy();
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let total_value: U256 = 100.into();
    let payment_time: U256 = 2.into();
    let payment_rate: U256 = 30000.into();
    proxy.calculate_epoch(owner, total_value, payment_time, payment_rate);
    let res: U256 = proxy.result();
    println!("{:?}", res);
}

#[test]
fn test_enable_locker() {
    let (env, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let prepay_amount: U256 = U256::from(20);

    let token_owner: Key = Key::from(package_hash);
    let floor_asked: U256 = U256::from(1);
    let total_asked: U256 = U256::from(10000);
    let payment_time: U256 = U256::from(3);
    let payment_rate: U256 = U256::from(300);

    initialize(
        &env,
        owner,
        &proxy,
        token_owner,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
    );

    proxy.make_contribution(owner, 150.into(), token_owner);
    proxy.enable_locker(owner, prepay_amount);
}

#[test]
fn test_disable_locker() {
    let (env, owner, contract, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let token_owner: Key = Key::from(package_hash);
    let token_id: Vec<U256> = vec![1.into(), 2.into()];
    let mut token_metas: Vec<BTreeMap<String, String>> = Vec::default();
    let mut meta: BTreeMap<String, String> = BTreeMap::default();
    meta.insert("TOKEN-1".into(), "Metadata for token1".into());
    token_metas.push(meta);
    let mut meta: BTreeMap<String, String> = BTreeMap::default();
    meta.insert("TOKEN-2".into(), "Metadata for token2".into());
    token_metas.push(meta);
    let cep47 = deploy_cep47(&env, owner, BTreeMap::default());
    cep47.call_contract(
        owner,
        "mint",
        runtime_args! {
            "recipient" => Key::Hash(contract.package_hash()),
            "token_ids" => token_id.clone(),
            "token_metas" => token_metas,
        },
        0,
    );
    let token_address: Key = Key::Hash(cep47.package_hash());
    let token_owner: Key = token_owner;
    let floor_asked: U256 = U256::from(1);
    let total_asked: U256 = U256::from(1);
    let payment_time: U256 = U256::from(1000);
    let payment_rate: U256 = U256::from(10000);
    proxy.initialize(
        owner,
        token_id,
        token_address,
        token_owner,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
    );
    proxy.disable_locker(owner);
}

#[test]
fn test_rescue_locker() {
    let (env, owner, contract, proxy) = deploy();
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    initialize(
        &env,
        owner,
        &proxy,
        Key::Hash(contract.package_hash()),
        1.into(),
        1.into(),
        1000.into(),
        1000.into(),
    );
    proxy.rescue_locker(owner);
}

#[test]
fn test_refund_due_disabled() {
    let (env, owner, _, proxy) = deploy();
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let token_owner: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();
    let floor_asked: U256 = U256::from(1);
    let total_asked: U256 = U256::from(200);
    let payment_time: U256 = U256::from(1000);
    let payment_rate: U256 = U256::from(10000);
    initialize(
        &env,
        owner,
        &proxy,
        token_owner,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
    );
    proxy.make_contribution(owner, 1000.into(), Key::Account(owner));
    let refund_address: Key = Key::Account(owner);
    proxy.refund_due_disabled(owner, refund_address);
}

#[test]
fn test_refund_due_single() {
    let (env, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let token_owner: Key = Key::from(package_hash);
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    initialize(
        &env,
        owner,
        &proxy,
        token_owner,
        1.into(),
        1.into(),
        1000.into(),
        1000.into(),
    );
    let token_amount = U256::from(100);
    proxy.make_contribution(owner, token_amount, Key::Account(owner));
    let refund_address: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000",
    )
    .unwrap();
    let cep47 = deploy_cep47(&env, owner, BTreeMap::default());
    let token_address: Key = Key::Hash(cep47.package_hash());
    let token_id: Vec<U256> = vec![1.into(), 2.into()];
    let floor_asked: U256 = U256::from(1);
    let total_asked: U256 = U256::from(100000);
    let payment_time: U256 = U256::from(1000);
    let payment_rate: U256 = U256::from(10000);
    let token_amount1 = U256::from(1000);
    proxy.initialize(
        owner,
        token_id,
        token_address,
        token_owner,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
    );
    proxy.make_contribution(owner, token_amount1, refund_address);
    proxy.refund_due_single(owner, refund_address);
}

#[test]
fn test_donate_funds() {
    let (env, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let token_owner: Key = Key::from(package_hash);
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    initialize(
        &env,
        owner,
        &proxy,
        token_owner,
        1.into(),
        1.into(),
        1000.into(),
        1000.into(),
    );
    let donation_amount: U256 = U256::from(1);
    proxy.donate_funds(owner, donation_amount);
}

#[test]
fn test_pay_back_funds() {
    let (env, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let prepay_amount: U256 = U256::from(2000);

    let token_owner: Key = Key::from(package_hash);
    let floor_asked: U256 = U256::from(1000);
    let total_asked: U256 = U256::from(950000000);
    let payment_time: U256 = U256::from(99900000);
    let payment_rate: U256 = U256::from(30);

    initialize(
        &env,
        owner,
        &proxy,
        token_owner,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
    );

    proxy.make_contribution(owner, 2146000000000u64.into(), token_owner);
    proxy.enable_locker(owner, prepay_amount);
    proxy.pay_back_funds(owner, 2146000000000u64.into());
}

#[test]
fn test_liquidate_locker() {
    let (env, owner, contract, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let cep47 = deploy_cep47(&env, owner, BTreeMap::default());
    let token_address: Key = Key::Hash(cep47.package_hash());
    let token_id: Vec<U256> = vec![1.into(), 2.into()];
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
            "token_ids" => token_id.clone(),
            "token_metas" => token_metas,
        },
        0,
    );
    cep47.call_contract(
        owner,
        "transfer",
        runtime_args! {
            "recipient" => Key::Hash(contract.package_hash()),
            "token_ids" => token_id.clone(),
        },
        0,
    );
    let token_owner: Key = Key::from(package_hash);
    let floor_asked: U256 = U256::from(0);
    let total_asked: U256 = U256::from(0);
    let payment_time: U256 = U256::from(0);
    let payment_rate: U256 = U256::from(0);

    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    proxy.initialize(
        owner,
        token_id,
        token_address,
        token_owner,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
    );
    proxy.liquidate_locker(owner);
}

#[test]
fn test_claim_interest_single() {
    let (env, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let token_owner: Key = Key::from(package_hash);
    initialize(
        &env,
        owner,
        &proxy,
        token_owner,
        1.into(),
        1.into(),
        1000.into(),
        1000.into(),
    );
    let token_amount = U256::from(100);
    proxy.donate_funds(owner, 1000.into());
    proxy.make_contribution(owner, token_amount, token_owner);
    proxy.claim_interest_single(owner);
}

#[test]
fn test_claim_interest_public() {
    let (env, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let token_amount = U256::from(1000);
    let cep47 = deploy_cep47(&env, owner, BTreeMap::default());
    let token_address: Key = Key::Hash(cep47.package_hash());
    let token_id: Vec<U256> = vec![1.into(), 2.into(), 3.into(), 4.into(), 5.into()];
    let token_owner: Key = Key::from(package_hash);
    let floor_asked: U256 = U256::from(1);
    let total_asked: U256 = U256::from(100000);
    let payment_time: U256 = U256::from(1);
    let payment_rate: U256 = U256::from(1);
    proxy.initialize(
        owner,
        token_id,
        token_address,
        token_owner,
        floor_asked,
        total_asked,
        payment_time,
        payment_rate,
    );
    proxy.donate_funds(owner, 1000.into());
    proxy.make_contribution(owner, token_amount, token_owner);
    proxy.claim_interest_public(owner);
}
