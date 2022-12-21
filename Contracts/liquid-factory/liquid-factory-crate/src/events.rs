use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum LiquidFactoryEvent {
    NewLocker {
        locker_address: Key,
        owners_address: Key,
        tokens_address: Key,
    },
    ContributeToLocker {
        locker_address: Key,
        backer_address: Key,
        contribution_amount: U256,
        total_increase_amount: U256,
    },
    DonateToLocker {
        locker_address: Key,
        payers_address: Key,
        donate_amount: U256,
    },
    PaybackToLocker {
        locker_address: Key,
        payers_address: Key,
        payback_amount: U256,
    },
}

impl LiquidFactoryEvent {
    pub fn type_name(&self) -> String {
        match self {
            LiquidFactoryEvent::NewLocker {
                locker_address: _,
                owners_address: _,
                tokens_address: _,
            } => "newLocker",
            LiquidFactoryEvent::ContributeToLocker {
                locker_address: _,
                backer_address: _,
                contribution_amount: _,
                total_increase_amount: _,
            } => "contributeToLocker",
            LiquidFactoryEvent::DonateToLocker {
                locker_address: _,
                payers_address: _,
                donate_amount: _,
            } => "donateToLocker",
            LiquidFactoryEvent::PaybackToLocker {
                locker_address: _,
                payers_address: _,
                payback_amount: _,
            } => "paybackToLocker",
        }
        .to_string()
    }
}
