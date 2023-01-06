use std::{collections::BTreeMap, time::SystemTime};

use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub struct LIQUIDFACTORYInstance(TestContract);

pub const ONE_MINUTE_IN_MS: u64 = 60000;
pub const ONE_DAY_IN_MS: u64 = 86400000;

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

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

impl LIQUIDFACTORYInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        default_token: Key,
        trustee_multisig: Key,
        time: u64,
    ) -> LIQUIDFACTORYInstance {
        LIQUIDFACTORYInstance(TestContract::new(
            env,
            "liquid-factory.wasm",
            contract_name,
            sender,
            runtime_args! {
                "default_token" => default_token,
                "trustee_multisig" => trustee_multisig,
            },
            time,
        ))
    }

    pub fn update_master(&self, sender: AccountHash, new_master: Key) {
        self.0.call_contract(
            sender,
            "update_master",
            runtime_args! {
                "new_master" => new_master
            },
            0,
        );
    }

    pub fn revoke_master(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "revoke_master", runtime_args! {}, 0);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_liquid_locker(
        &self,
        sender: AccountHash,
        token_id: Vec<U256>,
        token_address: Key,
        floor_asked: U256,
        delta_asked: U256,
        payment_time: U256,
        payment_rate: U256,
        payment_token: Key,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "create_liquid_locker_js_client",
            runtime_args! {
                "token_id" => token_id,
                "token_address" => token_address,
                "floor_asked" => floor_asked,
                "delta_asked" => delta_asked,
                "payment_time" => payment_time,
                "payment_rate" => payment_rate,
                "payment_token" => payment_token
            },
            time,
        );
    }

    pub fn create_empty_locker(&self, sender: AccountHash, payment_token: Key, time: u64) {
        self.0.call_contract(
            sender,
            "create_empty_locker_js_client",
            runtime_args! {
                "payment_token" => payment_token
            },
            time,
        );
    }

    pub fn contribute_to_locker(
        &self,
        sender: AccountHash,
        lockers_address: Key,
        payment_amount: U256,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "contribute_to_locker_js_client",
            runtime_args! {
                "lockers_address" => lockers_address,
                "payment_amount" => payment_amount
            },
            time,
        );
    }

    pub fn donate_to_locker(
        &self,
        sender: AccountHash,
        lockers_address: Key,
        donation_amount: U256,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "donate_to_locker",
            runtime_args! {
                "lockers_address" => lockers_address,
                "donation_amount" => donation_amount
            },
            time,
        );
    }

    pub fn payback_to_locker(
        &self,
        sender: AccountHash,
        lockers_address: Key,
        payment_amount: U256,
        time: u64,
    ) {
        self.0.call_contract(
            sender,
            "payback_to_locker",
            runtime_args! {
                "lockers_address" => lockers_address,
                "payment_amount" => payment_amount
            },
            time,
        );
    }

    // Result methods
    pub fn query<T: CLTyped + FromBytes>(&self, key: &str) -> T {
        self.0.query_named_key(key.into())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }
}

pub fn deploy_cep47(
    env: &TestEnv,
    owner: AccountHash,
    meta: BTreeMap<String, String>,
) -> TestContract {
    TestContract::new(
        env,
        "cep47-token.wasm",
        "cep47",
        owner,
        runtime_args! {
            "name" => "CEP",
            "symbol" => "CEP-47",
            "meta" => meta
        },
        now(),
    )
}

pub fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "erc20",
        owner,
        runtime_args! {
            "name" => "ERC",
            "symbol" => "ERC20",
            "decimals" => 9_u8,
            "initial_supply" => U256::from(0)
        },
        now(),
    )
}

pub fn deploy() -> (TestEnv, AccountHash, LIQUIDFACTORYInstance, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);

    let default_token: Key = Key::Hash(erc20.package_hash());

    let instance = LIQUIDFACTORYInstance::new(
        &env,
        "LIQUIDFACTORY",
        owner,
        default_token,
        Key::Account(owner),
        now(),
    );

    (env, owner, instance, erc20)
}

pub fn init() -> (
    TestEnv,
    Vec<AccountHash>,
    LIQUIDFACTORYInstance,
    TestContract,
    TestContract,
    Key,
    Key,
) {
    let (env, owner, factory_instance, erc20) = deploy();
    let cep47 = deploy_cep47(&env, owner, BTreeMap::default());
    let token_ids: Vec<U256> = vec![1.into(), 2.into()];

    let mut token_metas: Vec<BTreeMap<String, String>> = Vec::default();

    let mut meta: BTreeMap<String, String> = BTreeMap::default();
    meta.insert("TOKEN-1".into(), "Metadata for token1".into());
    token_metas.push(meta);

    let mut meta: BTreeMap<String, String> = BTreeMap::default();
    meta.insert("TOKEN-2".into(), "Metadata for token2".into());
    token_metas.push(meta);

    let accounts: Vec<AccountHash> = vec![owner, env.next_user(), env.next_user()];

    for account in &accounts {
        erc20.call_contract(
            *account,
            "mint",
            runtime_args! {
                "to" => Key::Account(*account),
                "amount" => U256::from(9_990_000_000_000u64)
            },
            now(),
        );

        erc20.call_contract(
            *account,
            "approve",
            runtime_args! {
                "spender" => Key::from(factory_instance.package_hash()),
                "amount" => U256::from(9_990_000_000_000u64),
            },
            now(),
        );
    }

    cep47.call_contract(
        owner,
        "mint",
        runtime_args! {
            "recipient" => Key::Account(owner),
            "token_ids" => token_ids.clone(),
            "token_metas" => token_metas,
        },
        now(),
    );

    cep47.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::from(factory_instance.package_hash()),
            "token_ids" => token_ids.clone(),
        },
        now(),
    );

    let token_address: Key = Key::Hash(cep47.package_hash());
    let floor_asked: U256 = 600_000_000_000u64.into();
    let delta_asked: U256 = 0.into();
    let payment_time: U256 = U256::from(86400000) * U256::from(300);
    let payment_rate: U256 = 10_000_000_000u64.into();
    let payment_token: Key = Key::Hash(erc20.package_hash());

    factory_instance.create_liquid_locker(
        owner,
        token_ids,
        token_address,
        floor_asked,
        delta_asked,
        payment_time,
        payment_rate,
        payment_token,
        now(),
    );

    let (lockers_contract_address, lockers_package_address): (Key, Key) =
        factory_instance.query("result");

    (
        env,
        accounts,
        factory_instance,
        erc20,
        cep47,
        lockers_contract_address,
        lockers_package_address,
    )
}
