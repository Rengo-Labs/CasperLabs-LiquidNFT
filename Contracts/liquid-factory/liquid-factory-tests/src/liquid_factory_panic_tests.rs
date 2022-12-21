use crate::liquid_factory_instance::*;
use casper_types::U256;

#[test]
#[should_panic]
fn should_not_be_able_to_contribute_after_contribution_phase_end() {
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
        now() + (ONE_MINUTE_IN_MS * 500_000_000),
    );
}
