use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub struct LIQUIDLOCKERInstance(TestContract);

impl LIQUIDLOCKERInstance {
    pub fn contract_instance(contract: TestContract) -> LIQUIDLOCKERInstance {
        LIQUIDLOCKERInstance(contract)
    }
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        trustee_multisig: Key,
        payment_token: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "liquid-locker.wasm",
            contract_name,
            sender,
            runtime_args! {
                "trustee_multisig" => trustee_multisig,
                "payment_token" => payment_token
            },
            0,
        )
    }
    pub fn proxy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        liquid_locker: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "liquid-locker-proxy.wasm",
            contract_name,
            sender,
            runtime_args! {"liquid_locker" => liquid_locker},
            0,
        )
    }
    pub fn liquid_locker(&self, sender: AccountHash, credentials: String) {
        self.0.call_contract(
            sender,
            "liquid_locker",
            runtime_args! {
                "credentials" => credentials
            },
            0,
        );
    }
    // Initialize Function
    #[allow(clippy::too_many_arguments)]
    pub fn initialize(
        &self,
        sender: AccountHash,
        token_id: Vec<U256>,
        token_address: Key,
        token_owner: Key,
        floor_asked: U256,
        total_asked: U256,
        payment_time: U256,
        payment_rate: U256,
    ) {
        self.0.call_contract(
            sender,
            "initialize",
            runtime_args! {
                "token_id" => token_id,
                "token_address" => token_address,
                "token_owner" => token_owner,
                "floor_asked" => floor_asked,
                "total_asked" => total_asked,
                "payment_time" => payment_time,
                "payment_rate" => payment_rate
            },
            0,
        );
    }
    // Increase Payment Rate
    pub fn increase_payment_rate(&self, sender: AccountHash, new_payment_rate: U256) {
        self.0.call_contract(
            sender,
            "increase_payment_rate",
            runtime_args! {
                "new_payment_rate" => new_payment_rate
            },
            0,
        );
    }
    // Make Contribution
    pub fn make_contribution(&self, sender: AccountHash, token_amount: U256, token_holder: Key) {
        self.0.call_contract(
            sender,
            "make_contribution",
            runtime_args! {
                "token_amount" => token_amount,
                "token_holder" => token_holder
            },
            0,
        );
    }
    // Decrease Payment Rate
    pub fn decrease_payment_time(&self, sender: AccountHash, new_payment_time: U256) {
        self.0.call_contract(
            sender,
            "decrease_payment_time",
            runtime_args! {
                "new_payment_time" => new_payment_time
            },
            0,
        );
    }
    // Enable Locker
    pub fn enable_locker(&self, sender: AccountHash, prepay_amount: U256) {
        self.0.call_contract(
            sender,
            "enable_locker",
            runtime_args! {
                "prepay_amount" => prepay_amount
            },
            0,
        );
    }
    //Disable Locker
    pub fn disable_locker(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "disable_locker", runtime_args! {}, 0);
    }
    //rescue_locker
    pub fn rescue_locker(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "rescue_locker", runtime_args! {}, 10000000000);
    }
    //refund_due_disabled
    pub fn refund_due_disabled(&self, sender: AccountHash, refund_address: Key) {
        self.0.call_contract(
            sender,
            "refund_due_disabled",
            runtime_args! {
                "refund_address" => refund_address
            },
            0,
        );
    }
    //refund_due_single
    pub fn refund_due_single(&self, sender: AccountHash, refund_address: Key) {
        self.0.call_contract(
            sender,
            "refund_due_single",
            runtime_args! {
                "refund_address" => refund_address
            },
            0,
        );
    }
    //donate_funds
    pub fn donate_funds(&self, sender: AccountHash, donation_amount: U256) {
        self.0.call_contract(
            sender,
            "donate_funds",
            runtime_args! {
                "donation_amount" => donation_amount
            },
            0,
        );
    }
    //pay_back_funds
    pub fn pay_back_funds(&self, sender: AccountHash, payment_amount: U256) {
        self.0.call_contract(
            sender,
            "pay_back_funds",
            runtime_args! {
                "payment_amount" => payment_amount
            },
            0,
        );
    }
    //pay_back_funds
    pub fn calculate_epoch(
        &self,
        sender: AccountHash,
        total_value: U256,
        payment_time: U256,
        payment_rate: U256,
    ) {
        self.0.call_contract(
            sender,
            "calculate_epoch",
            runtime_args! {
                "total_value" => total_value,
                "payment_time" => payment_time,
                "payment_rate" => payment_rate
            },
            0,
        );
    }
    //liquidate_locker
    pub fn liquidate_locker(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "liquidate_locker", runtime_args! {}, 100000000000);
    }
    //claim_interest_single
    pub fn claim_interest_single(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "claim_interest_single", runtime_args! {}, 0);
    }
    //claim_interest_public
    pub fn claim_interest_public(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "claim_interest_public", runtime_args! {}, 0);
    }

    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }
}
