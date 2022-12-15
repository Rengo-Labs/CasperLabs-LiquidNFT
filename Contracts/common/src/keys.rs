use casper_types::Key;

// Zero Key
pub fn zero_address() -> Key {
    Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000")
        .unwrap()
}
pub fn account_zero_address() -> Key {
    Key::from_formatted_str(
        "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap()
}
// Common Keys
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const RESULT: &str = "result";
/// Liquid Base
///How much a user has contributed to loan during contribution phase
pub const CONTRIBUTIONS_DICT: &str = "contributions";
///How much a user has received payed back for their potion of contributing to the loan
pub const COMPENSATIONS_DICT: &str = "compensations";
///Address of single provider, is zero address if there is no single provider
pub const SINGLE_PROVIDER: &str = "single_provider";
///Minimum the owner wants for the loan. If less than this contributors refunded
pub const FLOOR_ASKED: &str = "floor_asked";
///Maximum the owner wants for the loan
pub const TOTAL_ASKED: &str = "total_asked";
///How many tokens have been collected for far for this loan
pub const TOTAL_COLLECTED: &str = "total_collected";
///Balance contributors can claim at a given moment
pub const CLAIMABLE_BALANCE: &str = "claimable_balance";
///Balance the locker owner still owes
pub const REMAINING_BALANCE: &str = "remaining_balance";
///Balance of all penalties incurred by locker owner so far
pub const PENALTIES_BALANCE: &str = "penalties_balance";
///Time next payoff must happen to avoid penalties
pub const NEXT_DUE_TIME: &str = "next_due_time";
///Timestamp initialize was called
pub const CREATION_TIME: &str = "creation_time";
pub const FACTORY_ADDRESS: &str = "factory_address";
pub const PAYMENT_TOKEN: &str = "payment_token";
pub const TRUSTEE_MULTISIG: &str = "trustee_multisig";
pub const GLOBALS: &str = "globals";
/// Liquid Factory
pub const IMPLEMENTATIONS_DICT: &str = "implementations";
pub const LOCKERS_DICT: &str = "lockers";
pub const MASTER_ADDRESS: &str = "master_address";
pub const LOCKER_COUNT: &str = "locker_count";
pub const DEFAULT_COUNT: &str = "default_count";
pub const DEFAULT_TOKEN: &str = "default_token";
pub const COUNTER: &str = "counter";
