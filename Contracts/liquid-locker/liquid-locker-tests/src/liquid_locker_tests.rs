use crate::liquid_locker_instance::LIQUIDLOCKERInstance;
use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "erc2020",
        owner,
        runtime_args! {
            "name" => "ERC",
            "symbol" => "ERC20",
            "decimals" => 18 as u8,
            "initial_supply" => U256::from(1000000000000000 as u128)
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
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
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
    let package_hash: ContractPackageHash =
        contract.query_named_key("self_package_hash".to_string());
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => Key::from(package_hash),"amount"=> U256::from(2146000000)},
        0,
    );
    (env, owner, contract, proxy)
}

fn initialize(owner: AccountHash, instance: &LIQUIDLOCKERInstance, token_owner: Key) {
    let token_id: Vec<U256> = Vec::new();
    let token_address: Key = Key::Account(owner);
    let token_owner: Key = token_owner;
    let floor_asked: U256 = U256::from(1);
    let total_asked: U256 = U256::from(1);
    let payment_time: U256 = U256::from(1000);
    let payment_rate: U256 = U256::from(10000);
    instance.initialize(
        owner,
        token_id,
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
    let (_, owner, _, proxy) = deploy();
    let token_id: Vec<U256> = Vec::new();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let token_owner: Key = Key::from(package_hash);
    let token_address: Key = Key::Account(owner);
    let floor_asked: U256 = U256::from(1);
    let total_asked: U256 = U256::from(1);
    let payment_time: U256 = U256::from(1);
    let payment_rate: U256 = U256::from(1);
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
}

#[test]
fn test_increase_payment_rate() {
    let (_, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let token_owner: Key = Key::from(package_hash);
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    initialize(owner, &proxy, token_owner);
    let new_payment_rate: U256 = U256::from(1000000000);
    proxy.increase_payment_rate(owner, new_payment_rate);
}

#[test]
fn test_decrease_payment_time() {
    let (_, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let token_owner: Key = Key::from(package_hash);
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    initialize(owner, &proxy, token_owner);
    let new_payment_rate: U256 = U256::from(0);
    proxy.decrease_payment_time(owner, new_payment_rate);
}

#[test]
fn test_enable_locker() {
    let (_, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let prepay_amount: U256 = U256::from(20);
    let token_id: Vec<U256> = Vec::new();
    let token_owner: Key = Key::from(package_hash);
    let token_address: Key = Key::Account(owner);
    let floor_asked: U256 = U256::from(1);
    let total_asked: U256 = U256::from(10000);
    let payment_time: U256 = U256::from(3);
    let payment_rate: U256 = U256::from(300);
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
    proxy.make_contribution(owner, 150.into(), token_owner);
    proxy.enable_locker(owner, prepay_amount);
}

#[test]
fn test_disable_locker() {
    let (_, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let token_owner: Key = Key::from(package_hash);
    initialize(owner, &proxy, token_owner);
    proxy.disable_locker(owner);
}

#[test]
fn test_rescue_locker() {
    let (_, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let token_owner: Key = Key::from(package_hash);
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    initialize(owner, &proxy, token_owner);
    proxy.rescue_locker(owner);
}

#[test]
fn test_refund_due_disabled() {
    let (_, owner, _, proxy) = deploy();
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let token_id: Vec<U256> = Vec::new();
    let token_address: Key = Key::Account(owner);
    let token_owner: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    let floor_asked: U256 = U256::from(1);
    let total_asked: U256 = U256::from(200);
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
    proxy.make_contribution(owner, 1000.into(), Key::Account(owner));
    let refund_address: Key = Key::Account(owner);
    proxy.refund_due_disabled(owner, refund_address);
}

#[test]
fn test_refund_due_single() {
    let (_, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let token_owner: Key = Key::from(package_hash);
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    initialize(owner, &proxy, token_owner);
    let token_amount = U256::from(100);
    proxy.make_contribution(owner, token_amount, Key::Account(owner));
    let refund_address: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    let token_id: Vec<U256> = Vec::new();
    let token_address: Key = Key::Account(owner);
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
    let (_, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let token_owner: Key = Key::from(package_hash);
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    initialize(owner, &proxy, token_owner);
    let donation_amount: U256 = U256::from(1);
    proxy.donate_funds(owner, donation_amount);
}

#[test]
fn test_pay_back_funds() {
    let (_, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let token_id: Vec<U256> = Vec::new();
    let token_owner: Key = Key::from(package_hash);
    let token_address: Key = Key::Account(owner);
    let floor_asked: U256 = U256::from(1);
    let total_asked: U256 = U256::from(20000000);
    let payment_time: U256 = U256::from(10000);
    let payment_rate: U256 = U256::from(1);
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
    proxy.make_contribution(owner, 200000000.into(), token_owner);
    proxy.enable_locker(owner, 10.into());
    let payment_amount: U256 = U256::from(100000000);
    proxy.pay_back_funds(owner, payment_amount);
}

#[test]
fn test_liquidate_locker() {
    let (_, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let token_id: Vec<U256> = Vec::new();
    let token_address: Key = Key::Account(owner);
    let token_owner: Key = Key::from(package_hash);
    let floor_asked: U256 = U256::from(0);
    let total_asked: U256 = U256::from(0);
    let payment_time: U256 = U256::from(0);
    let payment_rate: U256 = U256::from(0);
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
    let (_, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let token_owner: Key = Key::from(package_hash);
    initialize(owner, &proxy, token_owner);
    let token_amount = U256::from(100);
    proxy.donate_funds(owner, 1000.into());
    proxy.make_contribution(owner, token_amount, token_owner);
    proxy.claim_interest_single(owner);
}

#[test]
fn test_claim_interest_public() {
    let (_, owner, _, proxy) = deploy();
    let package_hash: ContractPackageHash = proxy.query_named_key("package_hash".to_string());
    let proxy = LIQUIDLOCKERInstance::contract_instance(proxy);
    let token_amount = U256::from(1000);
    let token_id: Vec<U256> = Vec::new();
    let token_address: Key = Key::Account(owner);
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