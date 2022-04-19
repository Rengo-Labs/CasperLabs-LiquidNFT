use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum LiquidLockerEvent {
    SingleProvider { single_provider: Key },
    PaymentMade { payment_amount: U256 },
}

impl LiquidLockerEvent {
    pub fn type_name(&self) -> String {
        match self {
            LiquidLockerEvent::SingleProvider { single_provider: _ } => "singleProvider",
            LiquidLockerEvent::PaymentMade { payment_amount: _ } => "paymentMade",
        }
        .to_string()
    }
}
