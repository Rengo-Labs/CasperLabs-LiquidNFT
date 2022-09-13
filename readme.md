# CasperLabs-LiquidNFT

### Install the prerequisites

You can install the required software by issuing the following commands. If you are on an up-to-date Casper node, you probably already have all of the prerequisites installed so you can skip this step.

#### Note: If any command fails try again by restarting the terminal to reset the enviornment variable.


### Update package repositories
```
sudo apt update
```
### Install the command-line JSON processor
```
sudo apt install jq -y
```
### Install rust
Choose cutomize intallation to install nightly version
Install the nightly version (by default stable toolchain is installed)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
```
rustup install nightly-2022-08-29
```
### Check that nightly toolchain version is installed(this will list stable and nightly versions)
```
rustup toolchain list
```
### Set rust nightly as default
```
rustup default nightly-2022-08-29-x86_64-unknown-linux-gnu
```
### Install wasm32-unknown-unknown
```
rustup target add wasm32-unknown-unknown
```
### Rust Version
```
rustup --version
```
### Install Cmake
```
sudo apt-get -y install cmake
```
Note:https://cgold.readthedocs.io/en/latest/first-step/installation.html
### check if cmake is installed properly
```
cmake --version
```
### Install the Casper Crates
```
cargo install cargo-casper
```
### Add Casper repository

```
echo "deb https://repo.casperlabs.io/releases" bionic main | sudo tee -a /etc/apt/sources.list.d/casper.list
```
```
curl -O https://repo.casperlabs.io/casper-repo-pubkey.asc
```
```
sudo apt-key add casper-repo-pubkey.asc
```
```
sudo apt update
```
```
sudo apt install libssl-dev
```
```
sudo apt install pkg-config
```
### Install the Casper client software
```
cargo +nightly-2022-08-29-x86_64-unknown-linux-gnu install casper-client
```
### To check Casper Client Version
```
casper-client --version
```
# Additonal commands for help
```
casper-client --help
casper-client <command> --help
```

### Generate the keys

```
casper-client keygen keys

```
### Fund the key

The keys can be funded from casper live website [testnet faucet](https://testnet.cspr.live/tools/faucet). Requires chrome browser and the casper signer extension. You should import the keys that were generated in the previous step

## Build all the contracts and generate all required artifacts
```
make all
```
The above command also places all the keys in the folders as required. So make sure you have the key funded before running this command.
If you run out of funds and require a new pair of keys you can run the generate-key script target.

```
make prepare
make generate-key
```

Now head over to <br />
[Readme for JSClients](JsClients/readme.md)
The JSClients folder make it easier to interact with contracts with the help of automated scripts that do most of the heavy lifting. If you wish to manually test things out or want to deep dive into the contracts head over to the documentations page for contracts.

[Readme for Contracts](Contracts/README.md) <br />
