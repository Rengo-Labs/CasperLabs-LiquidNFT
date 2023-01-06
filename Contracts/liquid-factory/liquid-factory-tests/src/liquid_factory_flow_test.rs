use crate::liquid_factory_instance::*;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use test_env::call_contract_with_hash;

#[test]
fn test_contract_flow() {
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
        now(),
    );

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        now(),
    );
    let old_balance: U256 = erc20.query_named_key("balance".into());
    assert_eq!(old_balance, 10_389_800_000_000u64.into());

    factory_instance.payback_to_locker(
        accounts[0],
        lockers_package_address,
        200_000_000u64.into(),
        now(),
    );

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        now(),
    );
    let new_balance: U256 = erc20.query_named_key("balance".into());
    assert_eq!(
        new_balance,
        (10_389_800_000_000u64 - 200_000_000u64).into(),
        "Payback not done"
    );

    let donation_amount: U256 = 50.into();
    factory_instance.donate_to_locker(accounts[0], lockers_package_address, donation_amount, now());

    erc20.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        now(),
    );
    assert_eq!(
        new_balance - donation_amount,
        erc20.query_named_key("balance".into()),
        "Doantion not performed"
    );
}

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
        now(),
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
        now(),
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
        now(),
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
        now(),
    );

    cep47.call_contract(
        accounts[0],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        now(),
    );
    let balance: U256 = cep47.query_named_key("balance".into());
    assert_eq!(balance, 2.into(), "2 NFT's not returned to owner");
}

#[test]
fn should_be_able_to_liquadate_locker_as_owner_didnt_do_payment_for_days() {
    let (
        env,
        accounts,
        factory_instance,
        _erc20,
        cep47,
        lockers_contract_address,
        lockers_package_address,
    ) = init();
    let payment_amount: U256 = 5_000_000u64.into();
    factory_instance.contribute_to_locker(
        accounts[1],
        lockers_package_address,
        payment_amount,
        now(),
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

    // AS ASKED NOT REACHED SO LIQUIDATED TO TRUSTEE MULTISIG

    cep47.call_contract(
        accounts[1],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        now(),
    );
    let balance: U256 = cep47.query_named_key("balance".into());
    assert_eq!(balance, 0.into(), "Should not have NFT initially");

    call_contract_with_hash(
        &env,
        lockers_contract_address.into_hash().unwrap().into(),
        accounts[0],
        "liquidate_locker",
        runtime_args! {},
        now() + (ONE_MINUTE_IN_MS * 30000),
    );

    cep47.call_contract(
        accounts[1],
        "balance_of_js_client",
        runtime_args! { "owner" => Key::Account(accounts[0]) },
        now() + (ONE_MINUTE_IN_MS * 30000),
    );
    let balance: U256 = cep47.query_named_key("balance".into());
    assert_eq!(balance, 2.into(), "2 NFT's not liquidated to contributor");
}
