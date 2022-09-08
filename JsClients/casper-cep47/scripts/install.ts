import { config } from "dotenv";
config();
import { CEP47Client, utils, constants } from "../src";
import { parseTokenMeta, sleep, getDeploy } from "./utils";

import {
  CLValueBuilder,
  Keys,
  CLPublicKey,
  CLPublicKeyType,
} from "casper-js-sdk";

import * as fs from 'fs';
import { ContractPackageJson } from "casper-js-sdk/dist/lib/StoredValue";

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  //WASM_PATH,
  MASTER_KEY_PAIR_PATH,
  //TOKEN_NAME,
  //TOKEN_SYMBOL,
  CONTRACT_HASH,
  //INSTALL_PAYMENT_AMOUNT,
  MINT_ONE_PAYMENT_AMOUNT,
  MINT_COPIES_PAYMENT_AMOUNT,
  BURN_ONE_PAYMENT_AMOUNT,
  MINT_ONE_META_SIZE,
  MINT_COPIES_META_SIZE,
  MINT_COPIES_COUNT,
  MINT_MANY_META_SIZE,
  MINT_MANY_META_COUNT,
  //CONTRACT_NAME
} = process.env;

const TOKEN_NAME = "LNFT test";
const TOKEN_META = new  Map<string,string>().set("company name","Scytalelabs");
const TOKEN_SYMBOL = "LNFT";
const CONTRACT_NAME = "LNFT";
const INSTALL_PAYMENT_AMOUNT = "200000000000";
const WASM_PATH = "wasm/cep47-token.wasm";
const KEYS = Keys.Ed25519.parseKeyFiles(
  `${MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);


function getDeploymentCount() {
  return fs.readFileSync('deploymentCount','utf8');
}

function updateDeploymentCount() {
  let val:bigint = BigInt(fs.readFileSync('deploymentCount','utf8'));
  let newVal = val + BigInt(1);
  fs.writeFileSync('deploymentCount',newVal.toString(),{encoding:'utf8',flag:'w'});
}

const deployContract = async (tokenName: string = TOKEN_NAME,
                    tokenSymbol: string = TOKEN_SYMBOL, 
                    tokenMeta: Map<string,string> = TOKEN_META,
                    contractName: string = CONTRACT_NAME,
                    installPaymentAmount: string = INSTALL_PAYMENT_AMOUNT,
                    wasmPath: string = WASM_PATH
                    ) => {
  const cep47 = new CEP47Client(
    NODE_ADDRESS!,
    CHAIN_NAME!,
    EVENT_STREAM_ADDRESS!
  );
  
  contractName = contractName + getDeploymentCount();
  updateDeploymentCount();

  const installDeployHash = await cep47.install(
    KEYS,
    tokenName,
    tokenSymbol,
    tokenMeta,
    contractName,
    installPaymentAmount,
    wasmPath
  );

  console.log(`... Contract installation deployHash: ${installDeployHash}`);

  const response = await getDeploy(NODE_ADDRESS!, installDeployHash);

  console.log(`... Contract installed successfully.`);

  let accountInfo = await utils.getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);

  console.log(`... Account Info: `);
  console.log(JSON.stringify(accountInfo, null, 2));

  const contractHash = await utils.getAccountNamedKeyValue(
    accountInfo,
    `${contractName!}_contract_hash`
  );
  fs.writeFileSync('cep47ContractHash',contractHash,{encoding:'utf8',flag:'w'});
  fs.writeFileSync('../../LiquidNFT-Factory-Tests-Scripts/cep47ContractHash',contractHash,{encoding:'utf8',flag:'w'});
  console.log(`... Contract Hash: ${contractHash}`);

  const packageHash = await utils.getAccountNamedKeyValue(
    accountInfo,
    `${contractName!}_package_hash`
  );
  fs.writeFileSync('cep47PackageHash',packageHash,{encoding:'utf8',flag:'w'});
  fs.writeFileSync('../../LiquidNFT-Factory-Tests-Scripts/cep47PackageHash',packageHash,{encoding:'utf8',flag:'w'});
  console.log(`... Package Hash: ${packageHash}`);
};

function deployContractWithParams(){
  switch(process.argv.length) {
    case 2:{
      deployContract();
      break;
    }
    case 3: {
      deployContract(process.argv[2]);
      break;
    }
    case 4: {
      deployContract(process.argv[2],process.argv[3]);
      break;
    }
    case 5:{
      deployContract(process.argv[2],process.argv[3], new Map(parseTokenMeta(process.argv[4]!)));
      break;
    }
    case 6: {
      deployContract(process.argv[2],process.argv[3],new Map(parseTokenMeta(process.argv[4]!)),process.argv[5])
      break;
    }
    case 7: {
      deployContract(process.argv[2],process.argv[3],new Map(parseTokenMeta(process.argv[4]!)),process.argv[5],process.argv[6]);
      break;
    }
    case 8: {
      deployContract(process.argv[2],process.argv[3],new Map(parseTokenMeta(process.argv[4]!)),process.argv[5],process.argv[6],process.argv[7]);
      break;
    }
  }
}

//deployContractWithParams();