use alloc::{collections::BTreeMap, string::ToString, vec::Vec};
use casper_contract::{
    contract_api::{
        runtime::{self, get_blocktime},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, ContractPackageHash, Key, URef, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::errors::*;
use liquid_helper_crate::liquid_base_crate::{data::*, events::LiquidBaseEvent};
use liquid_helper_crate::LIQUIDHELPER;
use liquid_transfer_crate::LIQUIDTRANSFER;

pub trait LIQUIDLOCKER<Storage: ContractStorage>:
    ContractContext<Storage> + LIQUIDHELPER<Storage> + LIQUIDTRANSFER<Storage>
{
    fn init(
        &mut self,
        trustee_multisig: Key,
        payment_token: Key,
        factory_address: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        let mut g = get_globals();
        g.locker_owner = self.get_caller();
        set_globals(g);
        LIQUIDHELPER::init(
            self,
            factory_address,
            trustee_multisig,
            payment_token,
            contract_hash,
            package_hash,
        );
    }

    fn only_locker_owner(&self) {
        if self.get_caller() != get_globals().locker_owner {
            runtime::revert(ApiError::from(Error::InvalidOwner));
        }
    }

    fn only_from_factory(&self) {
        if self.get_caller() != get_factory_address() {
            runtime::revert(ApiError::from(Error::InvalidAddress));
        }
    }

    fn only_during_contribution_phase(&self) {
        if !(self.contribution_phase() && self.payment_time_not_set()) {
            runtime::revert(ApiError::from(Error::InvalidPhase));
        }
    }

    #[allow(clippy::too_many_arguments)]
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
        set_globals(Globals {
            token_id,
            payment_time,
            payment_rate,
            locker_owner: token_owner,
            token_address,
        });
        set_floor_asked(floor_asked);
        set_total_asked(total_asked);
        set_creation_time(U256::from(u64::from(get_blocktime())));
    }

    fn increase_payment_rate(&self, new_payment_rate: U256) {
        self.only_locker_owner();
        self.only_during_contribution_phase();

        if new_payment_rate > PRECISION_R {
            runtime::revert(ApiError::from(Error::InvalidRate));
        }

        if new_payment_rate <= get_globals().payment_rate {
            runtime::revert(ApiError::from(Error::InvalidIncrease));
        }

        let mut g = get_globals();
        g.payment_rate = new_payment_rate;
        set_globals(g);

        self.emit(&LiquidBaseEvent::PaymentRateIncrease { new_payment_rate });
    }

    fn decrease_payment_time(&self, new_payment_time: U256) {
        self.only_locker_owner();
        self.only_during_contribution_phase();

        if new_payment_time >= get_globals().payment_time {
            runtime::revert(ApiError::from(Error::InvalidDecrease));
        }

        let mut g = get_globals();
        g.payment_time = new_payment_time;
        set_globals(g);

        self.emit(&LiquidBaseEvent::PaymentTimeDecrease { new_payment_time });
    }

    fn update_settings(&mut self, new_payment_rate: U256, new_payment_time: U256) {
        self.only_locker_owner();
        self.only_during_contribution_phase();

        if new_payment_rate > PRECISION_R {
            runtime::revert(ApiError::from(Error::InvalidRate1));
        }

        if new_payment_rate <= get_globals().payment_rate {
            runtime::revert(ApiError::from(Error::InvalidRate2));
        }

        if new_payment_time >= get_globals().payment_time {
            runtime::revert(ApiError::from(Error::InvalidTime));
        }

        let mut g = get_globals();
        g.payment_rate = new_payment_rate;
        g.payment_time = new_payment_time;
        set_globals(g);

        self.emit(&LiquidBaseEvent::PaymentRateIncrease { new_payment_rate });
        self.emit(&LiquidBaseEvent::PaymentTimeDecrease { new_payment_time });
    }

    fn make_contribution(&mut self, token_amount: U256, token_holder: Key) -> (U256, U256) {
        self.only_from_factory();
        self.only_during_contribution_phase();

        let total_increase: U256 = self._total_increase(token_amount);
        let users_increase: U256 = self._users_increase(token_holder, token_amount, total_increase);
        self._increase_contributions(token_holder, users_increase);
        self._increase_total_collected(total_increase);

        (total_increase, users_increase)
    }

    /// @dev Check if this contribution adds enough for the user to become the sole contributor.
    /// Make them the sole contributor if so, otherwise return the totalAmount
    fn _users_increase(
        &mut self,
        token_holder: Key,
        token_amount: U256,
        total_amount: U256,
    ) -> U256 {
        if self.reached_total(token_holder, token_amount) {
            self._reached_total(token_holder)
        } else {
            total_amount
        }
    }

    /// @dev Calculate whether a contribution go over the maximum asked.
    /// If so only allow it to go up to the totalAsked an not over
    fn _total_increase(&self, token_amount: U256) -> U256 {
        if get_total_collected()
            .checked_add(token_amount)
            .unwrap_or_revert()
            < get_total_asked()
        {
            token_amount
        } else {
            get_total_asked()
                .checked_sub(get_total_collected())
                .unwrap_or_revert_with(Error::LiquidLockerUnderflowSub0)
        }
    }

    /// @dev Make the user the singleProvider.
    /// Making the user the singleProvider allows all other contributors to claim their funds back.
    /// Essentially if you contribute the whole maximum asked on your own you will kick everyone else out
    fn _reached_total(&mut self, token_holder: Key) -> U256 {
        if get_single_provider() != zero_address()
            && get_single_provider() != account_zero_address()
        {
            runtime::revert(ApiError::from(Error::ProviderExists));
        }

        let total_reach: U256 = get_total_asked()
            .checked_sub(Contributions::instance().get(&token_holder))
            .unwrap_or_revert_with(Error::LiquidLockerUnderflowSub1);

        set_single_provider(token_holder);

        self.emit(&LiquidBaseEvent::SingleProvider {
            single_provider: token_holder,
        });

        total_reach
    }

    fn enable_locker(&mut self, prepay_amount: U256) {
        self.only_locker_owner();

        if self.below_floor_asked() {
            runtime::revert(ApiError::from(Error::BelowFloor));
        }

        if !self.payment_time_not_set() {
            runtime::revert(ApiError::from(Error::EnabledLocker));
        }

        let (total_payback, epoch_payback, teams_payback): (U256, U256, U256) = self
            .calculate_paybacks(
                get_total_collected(),
                get_globals().payment_time,
                get_globals().payment_rate,
            );

        set_claimable_balance(
            get_claimable_balance()
                .checked_add(prepay_amount)
                .unwrap_or_revert(),
        );

        set_remaining_balance(
            total_payback
                .checked_sub(prepay_amount)
                .unwrap_or_revert_with(Error::LiquidLockerUnderflowSub2),
        );

        set_next_due_time(
            self.starting_timestamp()
                .checked_add(prepay_amount)
                .unwrap_or_revert()
                .checked_div(epoch_payback)
                .unwrap_or_revert_with(Error::LiquidLockerDivision0),
        );

        self._safe_transfer(
            get_payment_token(),
            self.get_caller(),
            get_total_collected()
                .checked_sub(prepay_amount)
                .unwrap_or_revert_with(Error::LiquidLockerUnderflowSub3)
                .checked_sub(teams_payback)
                .unwrap_or_revert_with(Error::LiquidLockerUnderflowSub4),
        );

        self._safe_transfer(get_payment_token(), get_trustee_multisig(), teams_payback);

        self.emit(&LiquidBaseEvent::PaymentMade {
            payment_amount: prepay_amount,
            payment_address: self.get_caller(),
        });
    }

    fn disable_locker(&self) {
        self.only_locker_owner();
        if !self.below_floor_asked() {
            runtime::revert(ApiError::from(Error::FloorReached));
        }
        self._return_owner_tokens();
    }

    /// @dev Internal function that does the work for disableLocker
    /// it returns all the NFT tokens to the original owner.
    fn _return_owner_tokens(&self) {
        let mut g = get_globals();
        let locker_owner = g.locker_owner;
        g.locker_owner = account_zero_address();
        set_globals(g);

        self.transfer_nft(
            get_globals().token_address,
            locker_owner,
            get_globals().token_id,
        );
    }

    fn rescue_locker(&self) {
        if self.get_caller() != get_trustee_multisig() {
            runtime::revert(ApiError::from(Error::InvalidTrustee));
        }

        if self.time_since(get_creation_time()) <= DEADLINE_TIME {
            runtime::revert(ApiError::from(Error::NotEnoughTime));
        }

        if !self.payment_time_not_set() {
            runtime::revert(ApiError::from(Error::AlreadyStarted));
        }

        self._return_owner_tokens();
    }

    fn refund_due_expired(&self, refund_address: Key) {
        if !(self.ownerless_locker() || self.floor_not_reached()) {
            runtime::revert(ApiError::from(Error::EnabledLocker));
        }

        let token_amount: U256 = Contributions::instance().get(&refund_address);

        self._refund_tokens(token_amount, refund_address);

        self._decrease_total_collected(token_amount);
    }

    fn refund_due_single(&self, refund_address: Key) {
        if !self.not_single_provider(refund_address) {
            runtime::revert(ApiError::from(Error::InvalidSender));
        }
        self._refund_tokens(
            Contributions::instance().get(&refund_address),
            refund_address,
        );
    }

    fn donate_funds(&self, donation_amount: U256) {
        self.only_from_factory();
        set_claimable_balance(
            get_claimable_balance()
                .checked_add(donation_amount)
                .unwrap_or_revert(),
        );
    }

    fn pay_back_funds(&mut self, payment_amount: U256, payment_address: Key) {
        self.only_from_factory();

        if !self.missed_deadline() {
            runtime::revert(ApiError::from(Error::TooLate));
        }

        self._adjust_balances(payment_amount, self._penalty_amount());

        self.emit(&LiquidBaseEvent::PaymentMade {
            payment_amount,
            payment_address,
        });

        if get_remaining_balance() == U256::from(0) {
            self._revoke_due_time();
            self._return_owner_tokens();
            return;
        }

        let mut payed_timestamp: U256 = get_next_due_time();
        let final_timestamp: U256 = self.payback_timestamp();

        if payed_timestamp == final_timestamp {
            return;
        }

        let purchased_time: U256 = payment_amount
            .checked_div(self.calculate_epoch(
                get_total_collected(),
                get_globals().payment_time,
                get_globals().payment_rate,
            ))
            .unwrap_or_revert_with(Error::LiquidLockerDivision1);

        if purchased_time < MILLI_SECONDS_IN_DAY {
            runtime::revert(ApiError::from(Error::MinimumPayoff));
        }

        payed_timestamp = if payed_timestamp > U256::from(u64::from(get_blocktime())) {
            payed_timestamp
                .checked_add(purchased_time)
                .unwrap_or_revert()
        } else {
            U256::from(u64::from(get_blocktime()))
                .checked_add(purchased_time)
                .unwrap_or_revert()
        };

        set_next_due_time(payed_timestamp);
    }

    fn liquidate_locker(&self) {
        if !(self.missed_activate() || self.missed_deadline()) {
            runtime::revert(ApiError::from(Error::TooEarly));
        }
        self._revoke_due_time();

        let mut g = get_globals();
        g.locker_owner = account_zero_address();
        set_globals(g);

        self.transfer_nft(
            get_globals().token_address,
            self.liquidate_to(),
            get_globals().token_id,
        );

        self.emit(&LiquidBaseEvent::Liquidated {
            liquidator_address: self.get_caller(),
        });
    }

    fn penalty_amount(&self, total_collected: U256, late_days_amount: U256) -> U256 {
        self._get_penalty_amount(total_collected, late_days_amount)
    }

    /// @dev calculate how much in penalties the owner has due to late time since last payment
    fn _penalty_amount(&self) -> U256 {
        self._get_penalty_amount(get_total_collected(), self.get_late_days())
    }

    /// @dev Calculate penalties. .5% for first 4 days and 1% for each day after the 4th
    fn _get_penalty_amount(&self, total_collected: U256, late_days_amount: U256) -> U256 {
        total_collected
            .checked_mul(self._days_base(late_days_amount))
            .unwrap_or_revert()
            .checked_div(200.into())
            .unwrap_or_revert_with(Error::LiquidLockerDivision2)
    }

    /// @dev Helper for the days math of calcualte penalties.
    /// Returns +1 per day before the 4th day and +2 for each day after the 4th day
    fn _days_base(&self, days_amount: U256) -> U256 {
        if days_amount > 4.into() {
            days_amount
                .checked_mul(2.into())
                .unwrap_or_revert()
                .checked_sub(4.into())
                .unwrap_or_revert_with(Error::LiquidLockerUnderflowSub5)
        } else {
            days_amount
        }
    }

    fn get_late_days(&self) -> U256 {
        if U256::from(u64::from(get_blocktime())) > get_next_due_time() {
            U256::from(u64::from(get_blocktime()))
                .checked_sub(get_next_due_time())
                .unwrap_or_revert_with(Error::LiquidLockerUnderflowSub6)
                .checked_div(MILLI_SECONDS_IN_DAY)
                .unwrap_or_revert_with(Error::LiquidLockerDivision3)
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
            .checked_add(PRECISION_R)
            .unwrap_or_revert()
            .checked_mul(total_value)
            .unwrap_or_revert()
            .checked_div(PRECISION_R)
            .unwrap_or_revert_with(Error::LiquidLockerDivision4);
        let teams_payback: U256 = total_payback
            .checked_sub(total_value)
            .unwrap_or_revert_with(Error::LiquidLockerUnderflowSub7)
            .checked_mul(FEE)
            .unwrap_or_revert()
            .checked_div(PRECISION_R)
            .unwrap_or_revert_with(Error::LiquidLockerDivision5);
        let epoch_payback: U256 = total_payback
            .checked_sub(total_value)
            .unwrap_or_revert_with(Error::LiquidLockerUnderflowSub8)
            .checked_div(payment_time)
            .unwrap_or_revert_with(Error::LiquidLockerDivision6);

        (total_payback, epoch_payback, teams_payback)
    }

    fn calculate_epoch(&self, total_value: U256, payment_time: U256, payment_rate: U256) -> U256 {
        total_value
            .checked_mul(payment_rate)
            .unwrap_or_revert()
            .checked_div(PRECISION_R)
            .unwrap_or_revert_with(Error::LiquidLockerDivision7)
            .checked_div(payment_time)
            .unwrap_or_revert_with(Error::LiquidLockerDivision8)
    }

    fn claim_interest(&self) {
        let provider = get_single_provider();

        if !(provider == zero_address()
            || provider == account_zero_address()
            || provider == self.get_caller())
        {
            runtime::revert(ApiError::from(Error::NotAuthorized));
        }

        self._claim_interest(self.get_caller());
    }

    /// @dev Does the internal work of claiming payed back tokens.
    /// Amount to claimed is based on share of contributions, and we record what someone has claimed in the
    /// compensations mapping
    fn _claim_interest(&self, claim_address: Key) {
        let claim_amount: U256 = get_claimable_balance()
            .checked_mul(Contributions::instance().get(&claim_address))
            .unwrap_or_revert()
            .checked_div(get_total_collected())
            .unwrap_or_revert_with(Error::LiquidLockerDivision9);

        let tokens_to_transfer = claim_amount
            .checked_sub(Compensations::instance().get(&claim_address))
            .unwrap_or_revert();

        Compensations::instance().set(&claim_address, claim_amount);

        self._safe_transfer(get_payment_token(), claim_address, tokens_to_transfer);

        self.emit(&LiquidBaseEvent::ClaimMade {
            claim_amount: tokens_to_transfer,
            claim_address,
        });
    }

    fn _refund_tokens(&self, refund_amount: U256, refund_address: Key) {
        Contributions::instance().set(
            &refund_address,
            Contributions::instance()
                .get(&refund_address)
                .checked_sub(refund_amount)
                .unwrap_or_revert(),
        );
        self._safe_transfer(get_payment_token(), refund_address, refund_amount);
        self.emit(&LiquidBaseEvent::RefundMade {
            refund_amount,
            refund_address,
        });
    }

    fn emit(&self, liquid_base_event: &LiquidBaseEvent) {
        let mut events = Vec::new();
        let package = get_contract_package_hash();
        match liquid_base_event {
            LiquidBaseEvent::SingleProvider { single_provider } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_base_event.type_name());
                event.insert("single_provider", single_provider.to_string());
                events.push(event);
            }
            LiquidBaseEvent::PaymentMade {
                payment_amount,
                payment_address,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_base_event.type_name());
                event.insert("payment_amount", payment_amount.to_string());
                event.insert("payment_address", payment_address.to_string());
                events.push(event);
            }
            LiquidBaseEvent::RefundMade {
                refund_amount,
                refund_address,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_base_event.type_name());
                event.insert("refund_amount", refund_amount.to_string());
                event.insert("refund_address", refund_address.to_string());
                events.push(event);
            }
            LiquidBaseEvent::ClaimMade {
                claim_amount,
                claim_address,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_base_event.type_name());
                event.insert("claim_amount", claim_amount.to_string());
                event.insert("claim_address", claim_address.to_string());
                events.push(event);
            }
            LiquidBaseEvent::Liquidated { liquidator_address } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_base_event.type_name());
                event.insert("liquidator_address", liquidator_address.to_string());
                events.push(event);
            }
            LiquidBaseEvent::PaymentRateIncrease { new_payment_rate } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_base_event.type_name());
                event.insert("new_payment_rate", new_payment_rate.to_string());
                events.push(event);
            }
            LiquidBaseEvent::PaymentTimeDecrease { new_payment_time } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", liquid_base_event.type_name());
                event.insert("new_payment_time", new_payment_time.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
