import { config } from "dotenv";
config();
import { CEP47Client, utils, constants } from "../src";
import { parseTokenMeta, sleep, getDeploy } from "./utils";


import {
  CLValueBuilder,
  Keys,
  CLPublicKey,
  CLAccountHash,
  CLPublicKeyType,
} from "casper-js-sdk";
import * as fs from 'fs';

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  WASM_PATH,
  MASTER_KEY_PAIR_PATH,
  RECEIVER_ACCOUNT_ONE,
  TOKEN_NAME,
  TOKEN_SYMBOL,
  CONTRACT_HASH,
  INSTALL_PAYMENT_AMOUNT,
  MINT_ONE_PAYMENT_AMOUNT,
  MINT_COPIES_PAYMENT_AMOUNT,
  BURN_ONE_PAYMENT_AMOUNT,
  MINT_ONE_META_SIZE,
  MINT_COPIES_META_SIZE,
  MINT_COPIES_COUNT,
  MINT_MANY_META_SIZE,
  MINT_MANY_META_COUNT,
  CONTRACT_NAME
} = process.env;


const KEYS = Keys.Ed25519.parseKeyFiles(
  `${MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

class Cep47 {
  contractHash: string;
  cep47Client: CEP47Client;
  constructor() {
    let _contractHash = fs.readFileSync('cep47ContractHash','utf8');
    this.contractHash = _contractHash.split("-").pop()!;
    this.cep47Client = new CEP47Client(
      NODE_ADDRESS!,
      CHAIN_NAME!,
      EVENT_STREAM_ADDRESS!
    );
  }

  mint = async (tokenIDsArray: Array<string>,metadata:Map<string,string>[]) => {
        
    await this.cep47Client.setContractHash(this.contractHash!);
    
    let mintDeployHash = await this.cep47Client.mint(
      KEYS,
      KEYS.publicKey,
      tokenIDsArray,
      metadata,
      MINT_ONE_PAYMENT_AMOUNT!);
    
    console.log("... Mint deploy hash: ", mintDeployHash);
    await getDeploy(NODE_ADDRESS!, mintDeployHash);
    console.log("... Token minted successfully.");
  }


  approve = async (spenderContractHash: string, tokenIdsArray: Array<string>) => {
      
    await this.cep47Client.setContractHash(this.contractHash!);

    const approveDeployHash = await this.cep47Client.approve(
      KEYS!,
      spenderContractHash!,
      tokenIdsArray!,
      MINT_ONE_PAYMENT_AMOUNT!
    );
    console.log("... Approve deploy hash: ", approveDeployHash);
  
    await getDeploy(NODE_ADDRESS!, approveDeployHash);
    console.log("... Token approved successfully");
  
  }

  totalSupply = async() => {
    await this.cep47Client.setContractHash(this.contractHash!);
    let _totalSupply = await this.cep47Client.totalSupply();
    console.log(`... Total supply: ${_totalSupply}`);
    return _totalSupply;
  } 
  
  name = async() => {
    await this.cep47Client.setContractHash(this.contractHash!);
    const _name = await this.cep47Client.name();
    console.log(`... Contract name: ${_name}`);
    return _name;
  }

  symbol = async() => {
    await this.cep47Client.setContractHash(this.contractHash!);
    const _symbol = await this.cep47Client.symbol();
    console.log(`... Contract symbol: ${_symbol}`);
    return _symbol;
  }

  meta = async() => {
    await this.cep47Client.setContractHash(this.contractHash!);
    const _meta = await this.cep47Client.meta();
    console.log(`... Contract meta: ${JSON.stringify(_meta)}`);
    return _meta;
  }
  
  balanceOf = async(publicKeyPemPath: string, secretKeyPemPath: string) => {
    
    await this.cep47Client.setContractHash(this.contractHash!);

    const _KEYS = Keys.Ed25519.parseKeyFiles(
      publicKeyPemPath,
      secretKeyPemPath
    );
    
    let _balance = await this.cep47Client.balanceOf(_KEYS.publicKey);
    console.log(`... Balance of account ${_KEYS.publicKey.toAccountHashStr()}`);
    console.log(`... Balance: ${_balance}`);
    return _balance;
  }

}

export{Cep47};