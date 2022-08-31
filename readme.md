# CasperLabs-LiquidNFT

## Readme for Contratcs
```
https://github.com/Scytalelabs-official/CasperLabs-LiquidNFT/blob/main/Contracts/README.md

```

## Readme for JSClient
```
https://github.com/Scytalelabs-official/CasperLabs-LiquidNFT/blob/main/JsClient/readme.md

```

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

## Generate the keys

Paste this command on the ubuntu terminal, that will create a keys folder for you containing public key , public key hex and secret key.

```
casper-client keygen keys

```
## Paste the keys

Paste the keys folder created by the above command to JsClient/LiquidNFT and JsClientForFrontend/LiquidNFT folders.

## Fund the key

We can fund the keys from casper live website faucet page on testnet.

Link:

```
https://testnet.cspr.live/tools/faucet

```
