use crate::data::{Compensations, Contributions};
use casperlabs_contract_utils::{ContractContext, ContractStorage};

pub trait LIQUIDBASE<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self) {
        Compensations::init();
        Contributions::init();
    }
}
