use casper_types::account::AccountHash;
use test_env::TestEnv;

use crate::liquid_helper_instance::LIQUIDHELPERInstance;

fn deploy() -> (TestEnv, AccountHash, LIQUIDHELPERInstance) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = LIQUIDHELPERInstance::new(&env, "LIQUIDHELPER", owner);
    (env, owner, instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
