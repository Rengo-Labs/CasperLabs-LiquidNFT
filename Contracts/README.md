# CasperLabs-LiquidNFT

Implementation of following contracts for the CasperLabs platform.

1. ERC20 Mock Contract
2. CEP47 Mock Contract
3. Liquid Base Crate
4. Liquid Transfer Crate
5. Liquid Factory Contract
6. Liquid Helper Contract
7. Liquid Locker Contract

## Table of contents

- [Interacting with the contract](#interacting-with-the-contract)
  - [Requirements](#requirements)
  - [Install the prerequisites](#install-the-prerequisites)
  - [Creating Keys](#creating-keys)
  - [Usage](#usage)
    - [Note](#note)
    - [Build Individual Smart Contract](#build-individual-smart-contract)
    - [Test individual Smart Contract](#test-individual-smart-contract)
    - [Run All Smart Contracts](#run-all-smart-contracts)
    - [Deploying Liquid Factory contract manually](#deploying-liquid-factory-contract-manually)
    - [Entry Point Methods](#LiquidFactory-entry-point-methods)
    - [Deploying Liquid Locker contract manually](#deploying-liquid-locker-contract-manually)
    - [Entry Point Methods](#LiquidLocker-entry-point-methods)

### Requirements

1. Install the [rust environment and casper client](https://docs.casperlabs.io/en/latest/.dapp-dev-guide/setup-of-rust-contract-sdk.html)

2. Install [wasm-strip](https://command-not-found.com/wasm-strip)

3. Clone this repo and navigate into the folder.

```bash
$ git clone https://github.com/Rengo-Labs-official/CasperLabs-LiquidNFT.git
```

4. A receiving Casper account. An easy way to set one up is using the [Casperlabs Signer](https://docs.cspr.community/docs/user-guides/SignerGuide.html).

### Install the prerequisites

You can install the required software by issuing the following commands. If you are on an up-to-date Casper node, you probably already have all of the prerequisites installed so you can skip this step.

#### Note: If any command fails try again by restarting the terminal to reset the enviornment variable.

```bash
# Update package repositories
sudo apt update
# Install the command-line JSON processor
sudo apt install jq -y
# Install rust
# Choose cutomize intallation to install nightly version
# Install the nightly version (by default stable toolchain is installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
# Check that nightly toolchain version is installed(this will list stable and nightly versions)
rustup toolchain list
# Set rust nightly as default
rustup default nightly
# Install wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown
# Rust Version
rustup --version
# Install Cmake
sudo apt-get -y install cmake
Note:https://cgold.readthedocs.io/en/latest/first-step/installation.html
# cmake Version
cmake --version
# Installing the Casper Crates
cargo install cargo-casper
# Add Casper repository
echo "deb https://repo.casperlabs.io/releases" bionic main | sudo tee -a /etc/apt/sources.list.d/casper.list
curl -O https://repo.casperlabs.io/casper-repo-pubkey.asc
sudo apt-key add casper-repo-pubkey.asc
sudo apt update
sudo apt install libssl-dev
sudo apt install pkg-config
# Install the Casper client software
cargo +nightly install casper-client
# To check Casper Client Version
casper-client --version
# Commands for help
casper-client --help
casper-client <command> --help
```

### Creating Keys

#### Note: Choose the name of directory where generated keys will be stored.

```bash
# Create keys
casper-client keygen <TARGET DIRECTORY>
```

### Usage

To run the Contracts make sure you are in the root folder.

#### Note

Make sure `wasm32-unknown-unknown` is installed.

```
make prepare
```

#### Run All Smart Contracts And Generate Keys

Run this command to build all smart contract and generate keys.

```
make all
```

#### Test All Smart Contracts

Run this command to build and test all smart contracts.

```
make test-all
```

#Alternatively you can build and test individual contracts

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

#### Build individual Smart Contract

You can run this commands to build individual smart contracts.

```
make build-contract-liquid-factory
make build-contract-liquid-locker
```

### Note: High processing power is required to run test cases otherwise you may face errors like

```
error: test failed
Caused by: process didn't exit successfully: `/home/.../.../ (signal: 9, SIGKILL: kill)
```

#### Test individual Smart Contract

You can run this commands to test individual smart contracts.

```
make test-liquid-factory
```

# Interacting with onchain contracts (the easy way)

A host of scripts have been made available in the package.json file. They simplify the task of running cumbersome manual commands. Make sure your in folder with package.json then run any script by

```
npm run <script_name> <param_one> <param_two> .....
```

### Deploying Liquid Factory contract manually

If you need to deploy the `Liquid Factory` contract manually you need to pass some parameters. Following is the command to deploy the `Liquid Factory contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="default_token:Key='default_token address'"
    --session-arg="trustee_multisig:Key='trustee_multisig address'"
    --session-arg="contract_name:string='contract_name'"
```

A successful response will look like:

```json
{
  "api_version": "1.0.0",
  "deploy_hash": "8c3068850354c2788c1664ac6a275ee575c8823676b4308851b7b3e1fe4e3dcc"
}
```

Once the network has received the deployment, it will queue up in the system before being listed in a block for execution. Sending a transaction (deployment) to the network does not mean that the transaction processed successfully. Therefore, it’s important to check to see that the contract executed properly, and that the block was finalized.

```bash
$ casper-client get-deploy --node-address http://<HOST:PORT> <DEPLOY_HASH>
```

Depending on your preference, it may be more convenient to just go to the cspr.live block explorer though after a minute or so:

```
https://testnet.cspr.live/deploy/<DEPLOY_HASH>
```

#### Sequence Diagram of Contribute to Locker

```
https://drive.google.com/file/d/1V4bJELB1duOq6-s7WI0VMZHS9rhzbsyL/view?usp=sharing
```

#### Contribute to Locker Flow

1. First we need to deply ERC20 Contract.
2. Call mint entry point using Erc20 Contract.
3. Deploy LiquidFactory Contract.
4. Call the approve function of Erc20 and give approval to liquid factory.
5. Call the create_empty_locker function of liquid factory.
6. At Last, call the contribute_to_locker function of liquid factory.

## Entry Point methods <a id="LiquidFactory-entry-point-methods"></a>

Following are the LiquidFactory's entry point methods.

- #### update_master <a id="LiquidFactory-update-master"></a>
  Transfer master permission.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| new_master     | Key  |

This method **returns** nothing.

- #### revoke_master <a id="LiquidFactory-revoke-master"></a>
  Destroy Master functionality.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### create_liquid_locker <a id="LiquidFactory-create-liquid-locker"></a>
  Call into initialize for the locker to begin the LiquidNFT loan process. Transfer the NFT the user wants use for the loan into the locker.

Following is the table of parameters.

| Parameter Name | Type        |
| -------------- | ----------- |
| token_id       | Vec`<U256>` |
| token_address  | Key         |
| floor_asked    | U256        |
| total_asked    | U256        |
| payment_time   | U256        |
| payment_rate   | U256        |
| payment_token  | Key         |

This method **returns** `(Key,Key)`.

- #### create_empty_locker <a id="LiquidFactory-create-empty-locker"></a>
  Creating an empty locker without any liquidity.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| payment_token  | Key  |

This method **returns** `(Key,Key)`.

- #### contribute_to_locker <a id="LiquidFactory-contribute-to-locker"></a>
  Call contributeToLocker. Factory acts as a middle man between the user and the locker. We do this so that the user only has to approve the factory and not each new locker.

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| lockers_address | Key  |
| payment_amount  | U256 |

This method **returns** `(U256,U256)`.

- #### donate_to_locker <a id="LiquidFactory-donate-to-locker"></a>
  Give tokens to a locker. These tokens do not go toward paying off the loan, they are instead distributed among the contributors for the loan. The result of this is that the value is transferred to the contributors not the owner because it does not deduct from the balance the owner owes.

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| lockers_address | Key  |
| donation_amount | U256 |

This method **returns** nothing.

- #### payback_to_locker <a id="LiquidFactory-payback-to-locker"></a>
  Call paybackToLocker. Factory acts as a middle man between the user and the locker. We do this so that the user only has to approve the factory and not each new locker.

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| lockers_address | Key  |
| payment_amount  | U256 |

This method **returns** nothing.

- #### initialize <a id="LiquidFactory-initialize"></a>
  This is a call made by the constructor to set up variables on a new locker. This is essentially equivalent to a constructor, but for our gas saving cloning operation instead. This may also be used in locker-reuse in version 2.

Following is the table of parameters.

| Parameter Name | Type        |
| -------------- | ----------- |
| token_id       | Vec`<U256>` |
| token_address  | Key         |
| token_owner    | Key         |
| floor_asked    | U256        |
| total_asked    | U256        |
| payment_time   | U256        |
| payment_rate   | U256        |

This method **returns** nothing.

- #### liquidate_locker <a id="LiquidFactory-liquidate-locker"></a>
  If the owner has missed payments by 7 days this call will transfer the NFT to either the singleProvider address or the trusted multisig to be auctioned.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### claim_interest <a id="LiquidFactory-claim-interest"></a>
  Claim payed back tokens

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### decrease_payment_time <a id="LiquidFactory-decrease-payment-time"></a>
  During the contribution phase, the owner can decrease the duration of the loan. The owner can only decrease the loan to a shorter duration, he cannot make it longer once the contribution phase has started.

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| new_payment_rate | U256 |

This method **returns** nothing.

- #### increase_payment_rate <a id="LiquidFactory-increase-payment-rate"></a>
  During the contribution phase, the owner can increase the rate they will pay for the loan. The owner can only increase the rate to make the deal better for contributors, he cannot decrease it.

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| new_payment_rate | U256 |

This method **returns** nothing.

- #### enable_locker <a id="LiquidFactory-enable-locker"></a>
  If the floor is reached early. The owner can also prepay an amount to pay off some of the earnings at enable time. The locker owner owes the earnings linearly until the end, then all of the actual loan plus any penalties are due at the end.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| prepay_amount  | U256 |

This method **returns** nothing.

- #### disable_locker <a id="LiquidFactory-disable-locker"></a>
  If the floor asked was not reached during contributions, this function will return the nft to the owner and allow all the contributors to claim their funds back.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### rescue_locker <a id="LiquidFactory-rescue-locker"></a>
  There are a couple edge cases with extreme payment rates that cause enableLocker to revert. These are never callable on our UI and doing so would require a manual transaction. This function will disable a locker in this senario, allow contributors to claim their money and transfer the NFT back to the owner. Only the team multisig has permission to do this.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### refund_due_expired <a id="LiquidFactory-refund-due-expired"></a>
  Allow users to claim funds when a locker is disabled.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| refund_address | Key  |

This method **returns** nothing.

- #### refund_due_single <a id="LiquidFactory-refund-due-single"></a>
  Allow users to claim funds when a someone kicks them out to become the single provider.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| refund_address | Key  |

This method **returns** nothing.

- #### donate_funds <a id="LiquidFactory-donate-funds"></a>
  Someone can add funds to the locker and they will be split among the contributors. This does not count as a payment on the loan.

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| donation_amount | U256 |

This method **returns** nothing.

- #### pay_back_funds <a id="LiquidFactory-pay-back-funds"></a>
  Locker owner can payback funds. Penalties are given if the owner does not pay the earnings linearally over the loan duration. If the owner pays back the earnings, loan amount, and penalties aka fully pays off the loan they will be transfered their nft back.

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| payment_amount  | U256 |
| payment_address | Key  |

This method **returns** nothing.

- #### calculate_epoch <a id="LiquidFactory-calculate-epoch"></a>
  Calculate how many sends should be added before the next payoff is due based on payment amount.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| total_value    | U256 |
| payment_time   | U256 |
| payment_rate   | U256 |

This method **returns** `U256`.

- #### calculate_paybacks <a id="LiquidFactory-calculate-paybacks"></a>
  Calulate how much the usage fee takes off a payments, and how many tokens are due per second of loan (epochPayback is amount of tokens to extend loan by 1 second. Only need to pay off earnings).

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| total_value    | U256 |
| payment_time   | U256 |
| payment_rate   | U256 |

This method **returns** `(U256, U256, U256)`.

- #### get_late_days <a id="LiquidFactory-get-late-days"></a>
  Helper for the days math of calcualte penalties. Returns +1 per day before the 4th day and +2 for each day after the 4th day.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `U256`.

- #### penalty_amount <a id="LiquidFactory-penalty-amount"></a>
  Public pure accessor for get_penalty_amount.

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| total_collected  | U256 |
| late_days_amount | U256 |

This method **returns** `U256`.

- #### penalty_amount <a id="LiquidFactory-penalty-amount"></a>
  Public users can add tokens to the pool to be used for the loan. The contributions for each user along with the total are recorded for splitting funds later. If a user contributes up to the maximum asked on a loan, they will become the sole provider (See users_increase and reached_total for functionality on becoming the sole provider). The sole provider will receive the token instead of the trusted multisig in the case if a liquidation.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| token_amount   | U256 |
| token_holder   | Key  |

This method **returns** `(U256, U256)`.

The params are documented in the package.json for each script

### Deploying Liquid Locker contract manually

If you need to deploy the `Liquid Locker` contract manually you need to pass some parameters. Following is the command to deploy the `Liquid Locker contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 170000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="trustee_multisig:Key='trustee-multisig-hash'"
    --session-arg="payment_token:Key='payment-token-hash'"
    --session-arg="contract_name:string='contract_name'"
```

A successful response will look like:

```json
{
  "api_version": "1.0.0",
  "deploy_hash": "8c3068850354c2788c1664ac6a275ee575c8823676b4308851b7b3e1fe4e3dcc"
}
```

Once the network has received the deployment, it will queue up in the system before being listed in a block for execution. Sending a transaction (deployment) to the network does not mean that the transaction processed successfully. Therefore, it’s important to check to see that the contract executed properly, and that the block was finalized.

```bash
$ casper-client get-deploy --node-address http://<HOST:PORT> <DEPLOY_HASH>
```

Depending on your preference, it may be more convenient to just go to the cspr.live block explorer though after a minute or so:

```
https://testnet.cspr.live/deploy/<DEPLOY_HASH>
```

## Entry Point methods <a id="LiquidLocker-entry-point-methods"></a>

Following are the LiquidLocker's entry point methods.

- #### initialize <a id="LiquidLocker-initialize"></a>
  This is a call made by the constructor to set up variables on a new locker. This is essentially equivalent to a constructor, but for our gas saving cloning operation instead. This may also be used in locker-reuse in version 2.

Following is the table of parameters.

| Parameter Name | Type        |
| -------------- | ----------- |
| token_id       | Vec`<U256>` |
| token_address  | Key         |
| token_owner    | Key         |
| floor_asked    | U256        |
| total_asked    | U256        |
| payment_time   | U256        |
| payment_rate   | U256        |

This method **returns** nothing.

- #### liquidate_locker <a id="LiquidLocker-liquidate-locker"></a>
  If the owner has missed payments by 7 days this call will transfer the NFT to either the singleProvider address or the trusted multisig to be auctioned.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### claim_interest <a id="LiquidLocker-claim-interest"></a>
  Claim payed back tokens

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### decrease_payment_time <a id="LiquidLocker-decrease-payment-time"></a>
  During the contribution phase, the owner can decrease the duration of the loan. The owner can only decrease the loan to a shorter duration, he cannot make it longer once the contribution phase has started.

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| new_payment_time | U256 |

This method **returns** nothing.

- #### increase_payment_rate <a id="LiquidLocker-increase-payment-rate"></a>
  During the contribution phase, the owner can increase the rate they will pay for the loan. The owner can only increase the rate to make the deal better for contributors, he cannot decrease it.

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| new_payment_rate | U256 |

This method **returns** nothing.

- #### enable_locker <a id="LiquidLocker-enable-locker"></a>
  If the floor is reached early. The owner can also prepay an amount to pay off some of the earnings at enable time. The locker owner owes the earnings linearly until the end, then all of the actual loan plus any penalties are due at the end.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| prepay_amount  | U256 |

This method **returns** nothing.

- #### disable_locker <a id="LiquidLocker-disable-locker"></a>
  If the floor asked was not reached during contributions, this function will return the nft to the owner and allow all the contributors to claim their funds back.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### rescue_locker <a id="LiquidLocker-rescue-locker"></a>
  There are a couple edge cases with extreme payment rates that cause enableLocker to revert. These are never callable on our UI and doing so would require a manual transaction. This function will disable a locker in this senario, allow contributors to claim their money and transfer the NFT back to the owner. Only the team multisig has permission to do this.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### refund_due_expired <a id="LiquidLocker-refund-due-expired"></a>
  Allow users to claim funds when a locker is disabled.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| refund_address | Key  |

This method **returns** nothing.

- #### refund_due_single <a id="LiquidLocker-refund-due-single"></a>
  Allow users to claim funds when a someone kicks them out to become the single provider.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| refund_address | Key  |

This method **returns** nothing.

- #### donate_funds <a id="LiquidLocker-donate-funds"></a>
  Someone can add funds to the locker and they will be split among the contributors. This does not count as a payment on the loan.

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| donation_amount | U256 |

This method **returns** nothing.

- #### pay_back_funds <a id="LiquidLocker-pay-back-funds"></a>
  Locker owner can payback funds. Penalties are given if the owner does not pay the earnings linearally over the loan duration. If the owner pays back the earnings, loan amount, and penalties aka fully pays off the loan they will be transfered their nft back.

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| payment_amount  | U256 |
| payment_address | Key  |

This method **returns** nothing.

- #### calculate_epoch <a id="LiquidLocker-calculate-epoch"></a>
  Calculate how many sends should be added before the next payoff is due based on payment amount.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| total_value    | U256 |
| payment_time   | U256 |
| payment_rate   | U256 |

This method **returns** `U256`.

- #### calculate_paybacks <a id="LiquidLocker-calculate-paybacks"></a>
  Calulate how much the usage fee takes off a payments, and how many tokens are due per second of loan (epochPayback is amount of tokens to extend loan by 1 second. Only need to pay off earnings).

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| total_value    | U256 |
| payment_time   | U256 |
| payment_rate   | U256 |

This method **returns** `(U256, U256, U256)`.

- #### get_late_days <a id="LiquidLocker-get-late-days"></a>
  Helper for the days math of calcualte penalties. Returns +1 per day before the 4th day and +2 for each day after the 4th day.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `U256`.

- #### penalty_amount <a id="LiquidLocker-penalty-amount"></a>
  Public pure accessor for get_penalty_amount.

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| total_collected  | U256 |
| late_days_amount | U256 |

This method **returns** `U256`.

- #### penalty_amount <a id="LiquidLocker-penalty-amount"></a>
  Public users can add tokens to the pool to be used for the loan. The contributions for each user along with the total are recorded for splitting funds later. If a user contributes up to the maximum asked on a loan, they will become the sole provider (See users_increase and reached_total for functionality on becoming the sole provider). The sole provider will receive the token instead of the trusted multisig in the case if a liquidation.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| token_amount   | U256 |
| token_holder   | Key  |

This method **returns** `(U256, U256)`.
