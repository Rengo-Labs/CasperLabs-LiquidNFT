use casper_types::account::AccountHash;
use test_env::TestEnv;

use crate::liquid_transfer_instance::LIQUIDTRANSFERInstance;

fn deploy() -> (TestEnv, AccountHash, LIQUIDTRANSFERInstance) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = LIQUIDTRANSFERInstance::new(&env, "LIQUIDTRANSFERInstance", owner);
    (env, owner, instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
