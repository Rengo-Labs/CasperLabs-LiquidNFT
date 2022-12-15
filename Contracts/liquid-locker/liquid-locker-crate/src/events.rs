use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum LiquidLockerEvent {
    SingleProvider { single_provider: Key },
    PaymentMade { payment_amount: U256 },
    PaymentRateIncrease { new_payment_rate: U256 },
    PaymentTimeDecrease { new_payment_time: U256 },
}

impl LiquidLockerEvent {
    pub fn type_name(&self) -> String {
        match self {
            LiquidLockerEvent::SingleProvider { single_provider: _ } => "singleProvider",
            LiquidLockerEvent::PaymentMade { payment_amount: _ } => "paymentMade",
            LiquidLockerEvent::PaymentRateIncrease {
                new_payment_rate: _,
            } => "paymentRateIncrease",
            LiquidLockerEvent::PaymentTimeDecrease {
                new_payment_time: _,
            } => "paymentTimeDecrease",
        }
        .to_string()
    }
}
