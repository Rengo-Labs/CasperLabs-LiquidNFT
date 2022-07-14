use casper_types::Key;

// Zero Key
pub fn zero_address() -> Key {
    Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000")
        .unwrap()
}
// Common Keys
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const RESULT: &str = "result";
// Liquid Base
pub const CONTRIBUTIONS_DICT: &str = "contributions";
pub const COMPENSATIONS_DICT: &str = "compensations";
pub const SINGLE_PROVIDER: &str = "single_provider";
pub const FLOOR_ASKED: &str = "floor_asked";
pub const TOTAL_ASKED: &str = "total_asked";
pub const TOTAL_COLLECTED: &str = "total_collected";
pub const CLAIMABLE_BALANCE: &str = "claimable_balance";
pub const REMAINING_BALANCE: &str = "remaining_balance";
pub const PENALTIES_BALANCE: &str = "penalties_balance";
pub const NEXT_DUE_TIME: &str = "next_due_time";
pub const CREATION_TIME: &str = "creation_time";
pub const PAYMENT_TOKEN: &str = "payment_token";
pub const TRUSTEE_MULTISIG: &str = "trustee_multisig";
pub const GLOBALS: &str = "globals";
// Liquid Factory
pub const IMPLEMENTATIONS_DICT: &str = "implementations";
pub const LOCKERS_DICT: &str = "lockers";
pub const MASTER_ADDRESS: &str = "master_address";
pub const LOCKER_COUNT: &str = "locker_count";
pub const DEFAULT_COUNT: &str = "default_count";
pub const DEFAULT_TOKEN: &str = "default_token";
pub const COUNTER: &str = "counter";
