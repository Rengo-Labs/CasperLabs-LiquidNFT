use crate::data::{
    set_factory_address, set_hash, set_package_hash, set_payment_token, set_trustee_multisig,
    Compensations, Contributions,
};
use casper_types::{ContractPackageHash, Key};
use casperlabs_contract_utils::{ContractContext, ContractStorage};

pub trait LIQUIDBASE<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &self,
        factory_address: Key,
        trustee_multisig: Key,
        payment_token: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        set_factory_address(factory_address);
        set_trustee_multisig(trustee_multisig);
        set_payment_token(payment_token);
        set_hash(contract_hash);
        set_package_hash(package_hash);
        Compensations::init();
        Contributions::init();
    }
}
