#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use casperlabs_cep47::{Meta, TokenId, CEP47};
use casperlabs_contract_utils::{set_key, ContractContext, OnChainContractStorage};

#[derive(Default)]
struct NFTToken(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for NFTToken {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl CEP47<OnChainContractStorage> for NFTToken {}
impl NFTToken {
    fn constructor(
        &mut self,
        name: String,
        symbol: String,
        meta: Meta,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        CEP47::init(self, name, symbol, meta, contract_hash, package_hash);
    }
}

#[no_mangle]
fn constructor() {
    let name = runtime::get_named_arg::<String>("name");
    let symbol = runtime::get_named_arg::<String>("symbol");
    let meta = runtime::get_named_arg::<Meta>("meta");
    let contract_hash = runtime::get_named_arg::<ContractHash>("contract_hash");
    let package_hash = runtime::get_named_arg::<ContractPackageHash>("package_hash");
    NFTToken::default().constructor(name, symbol, meta, contract_hash, package_hash);
}

#[no_mangle]
fn name() {
    let ret = NFTToken::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn symbol() {
    let ret = NFTToken::default().symbol();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn meta() {
    let ret = NFTToken::default().meta();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn total_supply() {
    let ret = NFTToken::default().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn balance_of() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let ret = NFTToken::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn balance_of_js_client() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let ret = NFTToken::default().balance_of(owner);
    set_key("balance", ret);
}

#[no_mangle]
fn get_token_by_index() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let index = runtime::get_named_arg::<U256>("index");
    let ret = NFTToken::default().get_token_by_index(owner, index);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn owner_of() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let ret = NFTToken::default().owner_of(token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn token_meta() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let ret = NFTToken::default().token_meta(token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn update_token_meta() {
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let token_meta = runtime::get_named_arg::<Meta>("token_meta");
    NFTToken::default()
        .set_token_meta(token_id, token_meta)
        .unwrap_or_revert();
}

#[no_mangle]
fn mint() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    let token_metas = runtime::get_named_arg::<Vec<Meta>>("token_metas");
    NFTToken::default()
        .mint(recipient, token_ids, token_metas)
        .unwrap_or_revert();
}

#[no_mangle]
fn mint_copies() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_ids = runtime::get_named_arg::<Vec<U256>>("token_ids");
    let token_meta = runtime::get_named_arg::<Meta>("token_meta");
    let count = runtime::get_named_arg::<u32>("count");
    NFTToken::default()
        .mint_copies(recipient, token_ids, token_meta, count)
        .unwrap_or_revert();
}

#[no_mangle]
fn burn() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    NFTToken::default()
        .burn(owner, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn transfer() {
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    NFTToken::default()
        .transfer(recipient, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn transfer_from() {
    let sender = runtime::get_named_arg::<Key>("sender");
    let recipient = runtime::get_named_arg::<Key>("recipient");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    NFTToken::default()
        .transfer_from(sender, recipient, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn approve() {
    let spender = runtime::get_named_arg::<Key>("spender");
    let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    NFTToken::default()
        .approve(spender, token_ids)
        .unwrap_or_revert();
}

#[no_mangle]
fn get_approved() {
    let owner = runtime::get_named_arg::<Key>("owner");
    let token_id = runtime::get_named_arg::<TokenId>("token_id");
    let ret = NFTToken::default().get_approved(owner, token_id);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("meta", Meta::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "meta",
        vec![],
        Meta::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of_js_client",
        vec![Parameter::new("owner", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "owner_of",
        vec![Parameter::new("token_id", TokenId::cl_type())],
        CLType::Option(Box::new(CLType::Key)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "token_meta",
        vec![Parameter::new("token_id", TokenId::cl_type())],
        Meta::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update_token_meta",
        vec![
            Parameter::new("token_id", TokenId::cl_type()),
            Parameter::new("token_meta", Meta::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
            Parameter::new("token_metas", CLType::List(Box::new(Meta::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint_copies",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
            Parameter::new("token_meta", Meta::cl_type()),
            Parameter::new("count", CLType::U32),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("sender", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("token_ids", CLType::List(Box::new(TokenId::cl_type()))),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_approved",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("token_id", TokenId::cl_type()),
        ],
        CLType::Option(Box::new(CLType::Key)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_token_by_index",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("index", U256::cl_type()),
        ],
        CLType::Option(Box::new(TokenId::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Store contract in the account's named keys. Contract name must be same for all new versions of the contracts
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        // add a first version to this package
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        let name: String = runtime::get_named_arg("name");
        let symbol: String = runtime::get_named_arg("symbol");
        let meta: Meta = runtime::get_named_arg("meta");
        let constructor_args = runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "meta" => meta,
            "contract_hash" => contract_hash,
            "package_hash" => package_hash
        };

        // Add the constructor group to the package hash with a single URef.
        let constructor_access: URef =
            storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert();

        // Call the constructor entry point
        let _: () =
            runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

        // Remove all URefs from the constructor group, so no one can call it for the second time.
        let mut urefs = BTreeSet::new();
        urefs.insert(constructor_access);
        storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
            .unwrap_or_revert();

        runtime::put_key(
            &format!("{}_package_hash", contract_name),
            package_hash.into(),
        );
        runtime::put_key(
            &format!("{}_package_hash_wrapped", contract_name),
            storage::new_uref(package_hash).into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
        runtime::put_key(
            &format!("{}_package_access_token", contract_name),
            access_token.into(),
        );
    }
    // If contract package did already exist
    else {
        // get the package
        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();
        // create new version and install it
        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}
