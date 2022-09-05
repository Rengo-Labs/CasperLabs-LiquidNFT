import { config } from "dotenv";
config();
import { LIQUIDNFTClient, utils} from "../src";
import { sleep, getDeploy } from "./utils";

import {
  Keys,
} from "casper-js-sdk";

import * as fs from 'fs';

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  LIQUIDNFT_WASM_PATH,
  LIQUIDNFT_MASTER_KEY_PAIR_PATH,
  LIQUIDNFT_INSTALL_PAYMENT_AMOUNT,
  LIQUIDNFT_CONTRACT_NAME,
  PAYMENTTOKEN_PACKAGE_HASH
} = process.env;

const KEYS = Keys.Ed25519.parseKeyFiles(
  `${LIQUIDNFT_MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${LIQUIDNFT_MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

function getDeploymentCount() {
  return fs.readFileSync('deploymentCount','utf8');
}

function updateDeploymentCount() {
  let val:bigint = BigInt(fs.readFileSync('deploymentCount','utf8'));
  let newVal = val + BigInt(1);
  fs.writeFileSync('deploymentCount',newVal.toString(),{encoding:'utf8',flag:'w'});
}

/*
@dev
params: default token
provide the payment tokenERC20 package hash
*/

const deployContract = async (defaultToken: string) => {
  const liquidNFT = new LIQUIDNFTClient(
    NODE_ADDRESS!,
    CHAIN_NAME!,
    EVENT_STREAM_ADDRESS!
  );

  let contractName = LIQUIDNFT_CONTRACT_NAME + getDeploymentCount();
  updateDeploymentCount();
  const installDeployHash = await liquidNFT.install(
    KEYS,
    // KEYS.publicKey,
    '1'!,
     defaultToken!,
     '0000000000000000000000000000000000000000000000000000000000000000',
    contractName!,
    LIQUIDNFT_INSTALL_PAYMENT_AMOUNT!,
    LIQUIDNFT_WASM_PATH!
  );

  console.log(`... Contract installation deployHash: ${installDeployHash}`);

  await getDeploy(NODE_ADDRESS!, installDeployHash);

  console.log(`... Contract installed successfully.`);

  let accountInfo = await utils.getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);

  console.log(`... Account Info: `);
  console.log(JSON.stringify(accountInfo, null, 2));

  const contractHash = await utils.getAccountNamedKeyValue(
    accountInfo,
    `${LIQUIDNFT_CONTRACT_NAME!}_contract_hash`
  );

  console.log(`... Contract Hash: ${contractHash}`);

  const packageHash = await utils.getAccountNamedKeyValue(
    accountInfo,
    `${LIQUIDNFT_CONTRACT_NAME!}_package_hash`
  );

  console.log(`... Package Hash: ${packageHash}`);
};

//deployContract();
