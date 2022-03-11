use crate::data;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, ContractPackageHash, Key, URef, U256, system::handle_payment::RuntimeProvider};
use contract_utils::{ContractContext, ContractStorage, set_key};
use liquid_helper::liquid_base_crate::{
    data::{DEADLINE_TIME, FEE, SECONDS_IN_DAY},
    LIQUIDBASE,
};
use liquid_helper::LIQUIDHELPER;
use liquid_nft_utils::commons::key_names::*;
use liquid_transfer_crate::{self, LiquidTransfer};

#[repr(u16)]
pub enum Error {
    InvalidOwner = 65,
    InvalidAddress,
    NotContributionPhase,
    InvalidIncrease,
    InvalidDecrease,
    ProviderExists,
    BelowFloor,
    EnabledLocker,
    FloorReached,
    InvalidTrustee,
    NotEnoughTime,
    AlreadyStarted,
    InvalidSender,
    TooLate,
    MinimumPayoff,
    TooEarly,
    NotSingleProvider,
    SingleProviderExists,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

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
pub trait LIQUIDLOCKER<Storage: ContractStorage>:
    ContractContext<Storage> + LiquidTransfer<Storage> + LIQUIDHELPER<Storage> + LIQUIDBASE<Storage>
{
    fn init(&mut self,payment_token:Key,trustee_multisig:Key,factory_address:Key, contract_hash: Key, package_hash: ContractPackageHash) {
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        LIQUIDBASE::init(self);
        LIQUIDBASE::set_factory_address(self, factory_address);
        LIQUIDBASE::set_payment_token(self, payment_token);
        LIQUIDBASE::set_trustee_multisig(self, trustee_multisig);
    }

    fn only_locker_owner(&self) {
        if !(self.get_caller() == LIQUIDBASE::Globals(self).get(LOCKER_OWNER)) {
            runtime::revert(ApiError::from(Error::InvalidOwner));
        }
    }

    fn only_from_factory(&self) {
        if !(self.get_caller() == LIQUIDBASE::get_factory_address(self)) {
            runtime::revert(ApiError::from(Error::InvalidAddress));
        }
    }
    fn only_during_contribution_phase(&self) {
        if !(LIQUIDHELPER::contribution_phase(self) == true
            && LIQUIDHELPER::payment_time_not_set(self) == true)
        {
            runtime::revert(ApiError::from(Error::NotContributionPhase));
        }
    }

    fn initialize(
        &self,
        token_id: Vec<U256>,
        token_address: Key,
        token_owner: Key,
        floor_asked: U256,
        total_asked: U256,
        payment_time: U256,
        payment_rate: U256,
    ) {
        self.only_from_factory();

        LIQUIDBASE::Globals(self).set(TOKEN_ID, token_id);
        LIQUIDBASE::Globals(self).set(LOCKER_OWNER, token_owner);
        LIQUIDBASE::Globals(self).set(TOKEN_ADDRESS, token_address);
        LIQUIDBASE::Globals(self).set(PAYMENT_TIME, payment_time);
        LIQUIDBASE::Globals(self).set(PAYMENT_RATE, payment_rate);

        LIQUIDBASE::set_floor_asked(self, floor_asked);
        LIQUIDBASE::set_total_asked(self, total_asked);

        if LIQUIDBASE::get_creation_time(self) > 0.into() {
            self._reset_values();
        }
        let blocktime: u64 = runtime::get_blocktime().into();
        LIQUIDBASE::set_creation_time(self, U256::from(blocktime));
    }

    fn _reset_values(&self) {
        LIQUIDBASE::set_claimable_balance(self, 0.into());
        LIQUIDBASE::set_remaining_balance(self, 0.into());
        LIQUIDBASE::set_penalties_balance(self, 0.into());
    }

    fn increase_payment_rate(&self, new_payment_rate: U256) {
        self.only_locker_owner();
        self.only_during_contribution_phase();
        if !(new_payment_rate > LIQUIDBASE::Globals(self).get(PAYMENT_RATE)) {
            runtime::revert(ApiError::from(Error::InvalidIncrease));
        }

        LIQUIDBASE::Globals(self).set(PAYMENT_RATE, new_payment_rate);
    }

    fn decrease_payment_time(&self, new_payment_time: U256) {
        self.only_locker_owner();
        self.only_during_contribution_phase();

        if !(new_payment_time < LIQUIDBASE::Globals(self).get(PAYMENT_TIME)) {
            runtime::revert(ApiError::from(Error::InvalidDecrease));
        }

        LIQUIDBASE::Globals(self).set(PAYMENT_TIME, new_payment_time);
    }

    fn make_contribution(&mut self, token_amount: U256, token_holder: Key) -> (U256, U256) {
       self.only_from_factory();
       self.only_during_contribution_phase();
        let total_increase: U256 = self._total_increase(token_amount);
        let users_increase: U256 = self._users_increase(token_holder, token_amount, total_increase);
        LIQUIDHELPER::_increase_contributions(self, token_holder, users_increase);
        LIQUIDHELPER::_increase_total_collected(self, total_increase);
        
        (total_increase, users_increase)
    }

    fn _users_increase(
        &mut self,
        token_holder: Key,
        token_amount: U256,
        total_amount: U256,
    ) -> U256 {
        if LIQUIDHELPER::reached_total(self, token_holder, token_amount) {
            self._reached_total(token_holder)
        } else {
            total_amount
        }
    }

    fn _total_increase(&self, token_amount: U256) -> U256 {
        if LIQUIDBASE::get_total_collected(self)
            .checked_add(token_amount)
            .unwrap_or_revert()
            < LIQUIDBASE::get_total_asked(self)
        {
            token_amount
        } else {
            token_amount
                .checked_sub(LIQUIDBASE::get_total_collected(self))
                .unwrap_or_revert()
        }
    }

    fn _reached_total(&mut self, token_holder: Key) -> U256 {
        if !(LIQUIDBASE::get_single_provider(self) == LIQUIDBASE::ZERO_ADDRESS(self)) {
            runtime::revert(ApiError::from(Error::ProviderExists));
        }

        let total_reach: U256 = LIQUIDBASE::get_total_asked(self)
            .checked_sub(LIQUIDBASE::Contributions(self).get(&token_holder))
            .unwrap_or_revert();

        LIQUIDBASE::set_single_provider(self, token_holder);

        self.liquid_locker_emit(&LiquidLockerEvent::SingleProvider {
            single_provider: token_holder,
        });

        total_reach
    }

    fn enable_locker(&mut self, prepay_amount: U256) {
        self.only_locker_owner();
        if !(LIQUIDHELPER::below_floor_asked(self) == false) {
            runtime::revert(ApiError::from(Error::BelowFloor));
        }

        if !(LIQUIDHELPER::payment_time_not_set(self) == true) {
            runtime::revert(ApiError::from(Error::EnabledLocker));
        }

        let (total_payback, epoch_payback, teams_payback): (U256, U256, U256) = self
            .calculate_paybacks(
                LIQUIDBASE::get_total_collected(self),
                LIQUIDBASE::Globals(self).get(PAYMENT_TIME),
                LIQUIDBASE::Globals(self).get(PAYMENT_RATE),
            );
        LIQUIDBASE::set_claimable_balance(
            self,
            LIQUIDBASE::get_claimable_balance(self)
                .checked_add(prepay_amount)
                .unwrap_or_revert(),
        );
        LIQUIDBASE::set_remaining_balance(
            self,
            total_payback.checked_sub(prepay_amount).unwrap_or_revert(),
        );
        LIQUIDHELPER::_safe_transfer(
            self,
            LIQUIDBASE::get_payment_token(self),
            LIQUIDBASE::Globals(self).get(LOCKER_OWNER),
            LIQUIDBASE::get_total_collected(self)
                .checked_sub(prepay_amount)
                .unwrap_or_revert()
                .checked_sub(teams_payback)
                .unwrap_or_revert(),
        );
        LIQUIDHELPER::_safe_transfer(
            self,
            LIQUIDBASE::get_payment_token(self),
            LIQUIDBASE::get_trustee_multisig(self),
            teams_payback,
        );

        LIQUIDBASE::set_next_due_time(
            self,
            LIQUIDHELPER::starting_timestamp(self)
                .checked_add(prepay_amount)
                .unwrap_or_revert()
                .checked_div(epoch_payback)
                .unwrap_or_revert(),
        );
        self.liquid_locker_emit(&LiquidLockerEvent::PaymentMade {
            payment_amount: prepay_amount,
        });
    }
    fn disable_locker(&self) {
        self.only_locker_owner();
        if !(LIQUIDHELPER::below_floor_asked(self) == true) {
            runtime::revert(ApiError::from(Error::FloorReached));
        }
         self._disable_locker();
    }

    fn _disable_locker(&self) {
       self._return_token();
        LIQUIDHELPER::_revoke_owner(self);
    }

    fn rescue_locker(&self) {
        if !(self.get_caller() == LIQUIDBASE::get_trustee_multisig(self)) {
            runtime::revert(ApiError::from(Error::InvalidTrustee));
        }

        if !(LIQUIDHELPER::time_since(self, LIQUIDBASE::get_creation_time(self))
            > U256::from(DEADLINE_TIME))
        {
            runtime::revert(ApiError::from(Error::NotEnoughTime));
        }

        if !(LIQUIDHELPER::payment_time_not_set(self) == true) {
            runtime::revert(ApiError::from(Error::AlreadyStarted));
        }
      self._disable_locker();
    }

    fn refund_due_disabled(&self, refund_address: Key) {
        if !(LIQUIDHELPER::ownerless_locker(self) == true
            || LIQUIDHELPER::floor_not_reached(self) == true)
        {
            runtime::revert(ApiError::from(Error::EnabledLocker));
        }
        let token_amount: U256 = LIQUIDBASE::Contributions(self).get(&refund_address);
        self._refund_tokens(token_amount, refund_address);

        LIQUIDHELPER::_decrease_total_collected(self, token_amount);
    }

    fn refund_due_single(&self, refund_address: Key) {
        if !(LIQUIDHELPER::not_single_provider(self, refund_address) == true) {
            runtime::revert(ApiError::from(Error::InvalidSender));
        }
        self._refund_tokens(
            LIQUIDBASE::Contributions(self).get(&refund_address),
            refund_address,
        );
    }

    fn donate_funds(&self, donation_amount: U256) {
        self.only_from_factory();

        LIQUIDBASE::set_claimable_balance(
            self,
            LIQUIDBASE::get_claimable_balance(self)
                .checked_add(donation_amount)
                .unwrap_or_revert(),
        );
    }

    fn pay_back_funds(&mut self, payment_amount: U256) {
        self.only_from_factory();

        if !(LIQUIDHELPER::missed_deadline(self) == false) {
            runtime::revert(ApiError::from(Error::TooLate));
        }

        self._adjust_balances(payment_amount, self._penalty_amount());

        if LIQUIDBASE::get_remaining_balance(self) == U256::from(0) {
            self._disable_locker();
           LIQUIDHELPER::_revoke_due_time(self);
           self._split_penalties();
            return;
        }

        let mut payed_timestamp: U256 = LIQUIDBASE::get_next_due_time(self);
        let final_timestamp: U256 = LIQUIDHELPER::payback_timestamp(self);

        if payed_timestamp == final_timestamp {
            return;
        }

        let purchased_time: U256 = payment_amount
            .checked_div(self.calculate_epoch(
                LIQUIDBASE::get_total_collected(self),
                LIQUIDBASE::Globals(self).get(PAYMENT_TIME),
                LIQUIDBASE::Globals(self).get(PAYMENT_RATE),
            ))
            .unwrap_or_revert();
        if !(purchased_time >= U256::from(SECONDS_IN_DAY)) {
             runtime::revert(ApiError::from(Error::MinimumPayoff));
        }

        let blocktime: u64 = runtime::get_blocktime().into();
        if payed_timestamp > U256::from(blocktime) {
            payed_timestamp = LIQUIDHELPER::_add(self, payed_timestamp, purchased_time);
        } else {
            payed_timestamp = LIQUIDHELPER::_add(self, U256::from(blocktime), purchased_time);
        }

        if payed_timestamp < final_timestamp {
            LIQUIDBASE::set_next_due_time(self, payed_timestamp);
        } else {
            LIQUIDBASE::set_next_due_time(self, final_timestamp);
        }
        self.liquid_locker_emit(&LiquidLockerEvent::PaymentMade { payment_amount });
    }

    fn liquidate_locker(&self) {
        if !(LIQUIDHELPER::missed_activate(self) == true
    || LIQUIDHELPER::missed_deadline(self) == true)
        {
            runtime::revert(ApiError::from(Error::TooEarly));
        }
        let token_id: Vec<U256> = LIQUIDBASE::Globals(self).get(TOKEN_ID);
        let token_address: Key = LIQUIDBASE::Globals(self).get(TOKEN_ADDRESS);
        for i in token_id {
            LiquidTransfer::transfer_nft(
                self,
                data::get_hash(),
                LIQUIDHELPER::liquidate_to(self),
                token_address,
                i,
            );
        }

        LIQUIDHELPER::_revoke_due_time(self);
        self._claim_penalties();
    }

    fn penalty_amount(&self, total_collected: U256, late_days_amount: U256) -> U256 {
        self._get_penalty_amount(total_collected, late_days_amount)
    }

    fn _penalty_amount(&self) -> U256 {
        self._get_penalty_amount(LIQUIDBASE::get_total_collected(self), self.get_late_days())
    }

    fn _get_penalty_amount(&self, total_collected: U256, late_days_amount: U256) -> U256 {
        total_collected
            .checked_mul(self._days_base(late_days_amount))
            .unwrap_or_revert()
            .checked_div(200.into())
            .unwrap_or_revert()
    }

    fn _days_base(&self, days_amount: U256) -> U256 {
        if days_amount > 4.into() {
            days_amount
                .checked_mul(2.into())
                .unwrap_or_revert()
                .checked_sub(4.into())
                .unwrap_or_revert()
        } else {
            days_amount
        }
    }

    fn get_late_days(&self) -> U256 {
        let blocktime: u64 = runtime::get_blocktime().into();
        if U256::from(blocktime) > LIQUIDBASE::get_next_due_time(self) {
            U256::from(blocktime)
                .checked_sub(LIQUIDBASE::get_next_due_time(self))
                .unwrap_or_revert()
                .checked_div(SECONDS_IN_DAY.into())
                .unwrap_or_revert()
        } else {
            0.into()
        }
    }

    fn calculate_paybacks(
        &self,
        total_value: U256,
        payment_time: U256,
        payment_rate: U256,
    ) -> (U256, U256, U256) {
        let total_payback: U256 = payment_rate
            .checked_add(100.into())
            .unwrap_or_revert()
            .checked_mul(total_value)
            .unwrap_or_revert()
            .checked_div(100.into())
            .unwrap_or_revert();
        let teams_payback: U256 = total_payback
            .checked_sub(total_value)
            .unwrap_or_revert()
            .checked_mul(FEE.into())
            .unwrap_or_revert()
            .checked_div(100.into())
            .unwrap_or_revert();
        let epoch_payback: U256 = total_payback
            .checked_sub(total_value)
            .unwrap_or_revert()
            .checked_div(payment_time)
            .unwrap_or_revert();

        (total_payback, epoch_payback, teams_payback)
    }

    fn calculate_epoch(&self, total_value: U256, payment_time: U256, payment_rate: U256) -> U256 {
        total_value
            .checked_mul(payment_rate)
            .unwrap_or_revert()
            .checked_div(100.into())
            .unwrap_or_revert()
            .checked_div(payment_time)
            .unwrap_or_revert()
    }

    fn claim_interest_single(&self) {
        if !(LIQUIDBASE::get_single_provider(self) == self.get_caller()) {
            runtime::revert(ApiError::from(Error::NotSingleProvider));
        }

        self._claim_interest(self.get_caller());
    }

    fn claim_interest_public(&self) {
        if !(LIQUIDBASE::get_single_provider(self) == LIQUIDBASE::ZERO_ADDRESS(self)) {
            runtime::revert(ApiError::from(Error::SingleProviderExists));
        }

        self._claim_interest(self.get_caller());
    }

    fn _claim_interest(&self, claim_address: Key) {
        let claim_amount: U256 = LIQUIDBASE::get_claimable_balance(self)
            .checked_mul(LIQUIDBASE::Contributions(self).get(&claim_address))
            .unwrap_or_revert()
            .checked_div(LIQUIDBASE::get_total_collected(self))
            .unwrap_or_revert();

        LIQUIDHELPER::_safe_transfer(
            self,
            LIQUIDBASE::get_payment_token(self),
            claim_address,
            claim_amount
                .checked_sub(LIQUIDBASE::Compensations(self).get(&claim_address))
                .unwrap_or_revert(),
        );

        LIQUIDBASE::Compensations(self).set(&claim_address, claim_amount);
    }

    fn _claim_penalties(&self) {
        if !(LIQUIDBASE::get_penalties_balance(self) > LIQUIDBASE::get_claimable_balance(self)) {
            return;
        }

        LIQUIDHELPER::_safe_transfer(
            self,
            LIQUIDBASE::get_payment_token(self),
            LIQUIDBASE::get_trustee_multisig(self),
            LIQUIDBASE::get_penalties_balance(self),
        );

        LIQUIDBASE::set_claimable_balance(
            self,
            LIQUIDBASE::get_claimable_balance(self)
                .checked_sub(LIQUIDBASE::get_penalties_balance(self))
                .unwrap_or_revert(),
        );

        LIQUIDBASE::set_penalties_balance(self, 0.into());
    }

    fn _split_penalties(&self) {
        let team_balance: U256 = LIQUIDBASE::get_penalties_balance(self)
            .checked_mul(FEE.into())
            .unwrap_or_revert()
            .checked_div(100.into())
            .unwrap_or_revert();

        if team_balance > LIQUIDBASE::get_claimable_balance(self) {
            return;
        }

        LIQUIDHELPER::_safe_transfer(
            self,
            LIQUIDBASE::get_payment_token(self),
            LIQUIDBASE::get_trustee_multisig(self),
            team_balance,
        );

        LIQUIDBASE::set_claimable_balance(
            self,
            LIQUIDBASE::get_claimable_balance(self)
                .checked_sub(team_balance)
                .unwrap_or_revert(),
        );

        LIQUIDBASE::set_penalties_balance(self, 0.into());
    }

    fn _adjust_balances(&self, payment_tokens: U256, penalty_tokens: U256) {
        LIQUIDBASE::set_claimable_balance(
            self,
            LIQUIDBASE::get_claimable_balance(self)
                .checked_add(payment_tokens)
                .unwrap_or_revert(),
        );
        LIQUIDBASE::set_penalties_balance(
            self,
            LIQUIDBASE::get_penalties_balance(self)
                .checked_add(penalty_tokens)
                .unwrap_or_revert(),
        );

        LIQUIDBASE::set_remaining_balance(
            self,
            LIQUIDBASE::get_remaining_balance(self)
                .checked_sub(payment_tokens)
                .unwrap_or_revert()
                .checked_add(penalty_tokens)
                .unwrap_or_revert(),
        );
    }

    fn _return_token(&self) {
        let token_id: Vec<U256> = LIQUIDBASE::Globals(self).get(TOKEN_ID);
        let token_address: Key = LIQUIDBASE::Globals(self).get(TOKEN_ADDRESS);
        let locker_owner: Key = LIQUIDBASE::Globals(self).get(LOCKER_OWNER);
        for i in token_id {
            LiquidTransfer::transfer_nft(self, data::get_hash(), locker_owner, token_address, i);
        }
    }

    fn _refund_tokens(&self, refund_amount: U256, refund_address: Key) {
        LIQUIDBASE::Contributions(self).set(&refund_address, 0.into());

        LIQUIDHELPER::_safe_transfer(
            self,
            LIQUIDBASE::get_payment_token(self),
            refund_address,
            refund_amount,
        );
    }

    fn liquid_locker_emit(&mut self, liquid_locker_event: &LiquidLockerEvent) {
        let mut events = Vec::new();
        let package = data::get_contract_package_hash();
        match liquid_locker_event {
            LiquidLockerEvent::SingleProvider { single_provider } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_locker_event.type_name());
                event.insert("single_provider", single_provider.to_string());
                events.push(event);
            }
            LiquidLockerEvent::PaymentMade { payment_amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_locker_event.type_name());
                event.insert("payment_amount", payment_amount.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
