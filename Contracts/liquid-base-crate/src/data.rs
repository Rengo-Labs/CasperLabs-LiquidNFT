use alloc::vec::Vec;
use casper_types::{Key, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casperlabs_contract_utils::{get_key, set_key, Dict};

pub use common::keys::*;

/// Precision factor for interest rate in orders of 1E9
pub const PRECISION_R: U256 = U256([100_000_000_000, 0, 0, 0]);
/// Team fee relative in orders of 1E9
pub const FEE: U256 = U256([20_000_000_000, 0, 0, 0]);
/// Time before a liquidation will occur
pub const DEADLINE_TIME: U256 = U256([604800000, 0, 0, 0]);
/// How long the contribution phase lasts
pub const CONTRIBUTION_TIME: U256 = U256([432000000, 0, 0, 0]);
/// Amount of milli seconds in one day unit
pub const MILLI_SECONDS_IN_DAY: U256 = U256([86400000, 0, 0, 0]);

/// @dev
/// @element tokenID: NFT IDs
/// @element tokenAddress: address of NFT contract
/// @element paymentTime: how long loan will last
/// @element paymentRate: how much must be paid for loan
/// @element lockerOwner: who is taking out loan
#[derive(Debug, Clone, CLTyped, ToBytes, FromBytes)]
pub struct Globals {
    pub token_id: Vec<U256>,
    pub payment_time: U256,
    pub payment_rate: U256,
    pub locker_owner: Key,
    pub token_address: Key,
}
impl Default for Globals {
    fn default() -> Self {
        Globals {
            token_id: Default::default(),
            payment_time: 0.into(),
            payment_rate: 0.into(),
            locker_owner: account_zero_address(),
            token_address: zero_address(),
        }
    }
}

pub fn set_globals(globals: Globals) {
    set_key(GLOBALS, globals);
}

pub fn get_globals() -> Globals {
    get_key(GLOBALS).unwrap_or_default()
}

/// How much a user has contributed to loan during contribution phase
pub struct Contributions {
    dict: Dict,
}

impl Contributions {
    pub fn instance() -> Contributions {
        Contributions {
            dict: Dict::instance(CONTRIBUTIONS_DICT),
        }
    }

    pub fn init() {
        Dict::init(CONTRIBUTIONS_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

/// How much a user has received payed back for their potion of contributing to the loan
pub struct Compensations {
    dict: Dict,
}

impl Compensations {
    pub fn instance() -> Compensations {
        Compensations {
            dict: Dict::instance(COMPENSATIONS_DICT),
        }
    }

    pub fn init() {
        Dict::init(COMPENSATIONS_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

/// Address if factory that creates lockers
pub fn set_factory_address(factory_address: Key) {
    set_key(FACTORY_ADDRESS, factory_address);
}

pub fn get_factory_address() -> Key {
    get_key(FACTORY_ADDRESS).unwrap_or_else(zero_address)
}

/// Address to tranfer NFT to in event of non singleProvider liquidation
pub fn set_trustee_multisig(trustee_multisig: Key) {
    set_key(TRUSTEE_MULTISIG, trustee_multisig);
}

pub fn get_trustee_multisig() -> Key {
    get_key(TRUSTEE_MULTISIG).unwrap_or_else(zero_address)
}

/// ERC20 used for payments of this locker
pub fn set_payment_token(payment_token: Key) {
    set_key(PAYMENT_TOKEN, payment_token);
}

pub fn get_payment_token() -> Key {
    get_key(PAYMENT_TOKEN).unwrap_or_else(zero_address)
}

/// Address of single provider, is zero address if there is no single provider
pub fn set_single_provider(single_provider: Key) {
    set_key(SINGLE_PROVIDER, single_provider);
}

pub fn get_single_provider() -> Key {
    get_key(SINGLE_PROVIDER).unwrap_or_else(zero_address)
}

/// Minimum the owner wants for the loan. If less than this contributors refunded
pub fn set_floor_asked(floor_asked: U256) {
    set_key(FLOOR_ASKED, floor_asked);
}

pub fn get_floor_asked() -> U256 {
    get_key(FLOOR_ASKED).unwrap_or_default()
}

/// Maximum the owner wants for the loan
pub fn set_total_asked(total_asked: U256) {
    set_key(TOTAL_ASKED, total_asked);
}

pub fn get_total_asked() -> U256 {
    get_key(TOTAL_ASKED).unwrap_or_default()
}

/// How many tokens have been collected for far for this loan
pub fn set_total_collected(total_collected: U256) {
    set_key(TOTAL_COLLECTED, total_collected);
}

pub fn get_total_collected() -> U256 {
    get_key(TOTAL_COLLECTED).unwrap_or_default()
}

/// Balance contributors can claim at a given moment
pub fn set_claimable_balance(claimable_balance: U256) {
    set_key(CLAIMABLE_BALANCE, claimable_balance);
}

pub fn get_claimable_balance() -> U256 {
    get_key(CLAIMABLE_BALANCE).unwrap_or_default()
}

/// Balance the locker owner still owes
pub fn set_remaining_balance(remaining_balance: U256) {
    set_key(REMAINING_BALANCE, remaining_balance);
}

pub fn get_remaining_balance() -> U256 {
    get_key(REMAINING_BALANCE).unwrap_or_default()
}

/// Time next payoff must happen to avoid penalties
pub fn set_next_due_time(next_due_time: U256) {
    set_key(NEXT_DUE_TIME, next_due_time);
}

pub fn get_next_due_time() -> U256 {
    get_key(NEXT_DUE_TIME).unwrap_or_default()
}

/// Timestamp initialize was called
pub fn set_creation_time(creation_time: U256) {
    set_key(CREATION_TIME, creation_time);
}

pub fn get_creation_time() -> U256 {
    get_key(CREATION_TIME).unwrap_or_default()
}
