use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum LiquidBaseEvent {
    /// Event for when the single provider is established
    SingleProvider { single_provider: Key },
    /// Event for when the loan payback is made
    PaymentMade {
        payment_amount: U256,
        payment_address: Key,
    },
    /// Event for when the contributor gets refunded
    RefundMade {
        refund_amount: U256,
        refund_address: Key,
    },
    /// Event for when the contributor claims funds
    ClaimMade {
        claim_amount: U256,
        claim_address: Key,
    },
    /// Event for when the loan is liquidated or defaulted
    Liquidated { liquidator_address: Key },
    /// Event for when the interest rate is increased
    PaymentRateIncrease { new_payment_rate: U256 },
    /// Event for when the payback time is decreased
    PaymentTimeDecrease { new_payment_time: U256 },
}

impl LiquidBaseEvent {
    pub fn type_name(&self) -> String {
        match self {
            LiquidBaseEvent::SingleProvider { single_provider: _ } => "singleProvider",
            LiquidBaseEvent::PaymentMade {
                payment_amount: _,
                payment_address: _,
            } => "paymentMade",
            LiquidBaseEvent::RefundMade {
                refund_amount: _,
                refund_address: _,
            } => "refundMade",
            LiquidBaseEvent::ClaimMade {
                claim_amount: _,
                claim_address: _,
            } => "claimMade",
            LiquidBaseEvent::Liquidated {
                liquidator_address: _,
            } => "liquidated",
            LiquidBaseEvent::PaymentRateIncrease {
                new_payment_rate: _,
            } => "paymentRateIncrease",
            LiquidBaseEvent::PaymentTimeDecrease {
                new_payment_time: _,
            } => "paymentTimeDecrease",
        }
        .to_string()
    }
}
