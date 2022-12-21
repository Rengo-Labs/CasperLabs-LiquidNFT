import { config } from "dotenv";
config();
import LIQUIDNFTFACTORYClientForFunctions from "../../../../JsClientsForFrontend/LiquidNFT-Factory/src/liquidNFTFactory";
import * as utils from "../../../../JsClientsForFrontend/LiquidNFT-Factory/src/utils";
import LIQUIDNFTFactoryClientForDeployment from "../src/liquidNFTFactory";
import { getDeploymentCount, updateDeploymentCount, getDeploy } from "./utils";

import {
  Keys,
} from "casper-js-sdk";

import * as fs from 'fs';

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  LIQUIDNFT_FACTORY_WASM_PATH,
  LIQUIDNFT_FACTORY_MASTER_KEY_PAIR_PATH,
  LIQUIDNFT_FACTORY_CONTRACT_NAME,
  LIQUIDNFT_FACTORY_INSTALL_PAYMENT_AMOUNT,
  LIQUIDNFT_FACTORY_FUNCTIONS_PAYMENT_AMOUNT,
  LIQUIDNFT_FACTORY_CREATEEMPTYLOCKER_FUNCTIONS_PAYMENT_AMOUNT,
  LIQUIDNFT_FACTORY_CREATELOCKER_FUNCTIONS_PAYMENT_AMOUNT
} = process.env;


const KEYS = Keys.Ed25519.parseKeyFiles(
  `${LIQUIDNFT_FACTORY_MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${LIQUIDNFT_FACTORY_MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

export const deployContract = async (defaultToken: string) => {
  const liquidNFT = new LIQUIDNFTFactoryClientForDeployment(
    NODE_ADDRESS!,
    CHAIN_NAME!,
    EVENT_STREAM_ADDRESS!
  );

  let contractName = LIQUIDNFT_FACTORY_CONTRACT_NAME + getDeploymentCount();
  updateDeploymentCount();
  const installDeployHash = await liquidNFT.install(
    KEYS,
    defaultToken!,
    contractName!,
    LIQUIDNFT_FACTORY_INSTALL_PAYMENT_AMOUNT!,
    LIQUIDNFT_FACTORY_WASM_PATH!
  );

  console.log(`... Contract installation deployHash: ${installDeployHash}`);

  await getDeploy(NODE_ADDRESS!, installDeployHash);

  console.log(`... Contract installed successfully.`);

  let accountInfo = await utils.getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);

  console.log(`... Account Info: `);
  console.log(JSON.stringify(accountInfo, null, 2));

  const contractHash = await utils.getAccountNamedKeyValue(
    accountInfo,
    `${contractName!}_contract_hash`
  );
  fs.writeFileSync('liquidNFTFactoryContractHash', contractHash, { encoding: 'utf8', flag: 'w' });
  fs.writeFileSync('.././LiquidNFT-Factory-Tests-Scripts/mainContractFlowScript/liquidNFTFactoryContractHash', contractHash, { encoding: 'utf8', flag: 'w' });

  console.log(`... Contract Hash: ${contractHash}`);

  const packageHash = await utils.getAccountNamedKeyValue(
    accountInfo,
    `${contractName!}_package_hash`
  );
  fs.writeFileSync('liquidNFTFactoryPackageHash', packageHash, { encoding: 'utf8', flag: 'w' });
  fs.writeFileSync('.././LiquidNFT-Factory-Tests-Scripts/mainContractFlowScript/liquidNFTFactoryPackageHash', packageHash, { encoding: 'utf8', flag: 'w' });
  console.log(`... Package Hash: ${packageHash}`);

};

class LiquidNFTFactory {
  contractHash: string;
  liquidNFTFactory: LIQUIDNFTFACTORYClientForFunctions;
  constructor() {
    let _contractHash = fs.readFileSync('liquidNFTFactoryContractHash', 'utf8');
    this.contractHash = _contractHash.split("-").pop()!;
    this.liquidNFTFactory = new LIQUIDNFTFACTORYClientForFunctions(
      NODE_ADDRESS!,
      CHAIN_NAME!,
      EVENT_STREAM_ADDRESS!
    );
  }

  //createLiquidLocker
  createLiquidLockerJsClient = async (
    tokenIdsArray: Array<string>,
    cep47PackageHash: string,
    floorAsked: string,
    totalAsked: string,
    paymentTime: string,
    paymentRate: string,
    erc20PackageHash: string,
  ) => {
    await this.liquidNFTFactory.setContractHash(this.contractHash!);
    const createLiquidLockerJsClientDeployHash = await this.liquidNFTFactory.createLiquidLockerJsClient(
      KEYS,
      tokenIdsArray!,
      cep47PackageHash!,
      floorAsked!,
      totalAsked!,
      paymentTime!,
      paymentRate!,
      erc20PackageHash!,
      LIQUIDNFT_FACTORY_CREATELOCKER_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... createLiquidLockerJsClient deploy hash: ", createLiquidLockerJsClientDeployHash);

    await getDeploy(NODE_ADDRESS!, createLiquidLockerJsClientDeployHash);
    console.log("... createLiquidLockerJsClient function called successfully");
  }

  //lockerHashes
  lockerHashes = async () => {
    await this.liquidNFTFactory.setContractHash(this.contractHash!);
    const result = await this.liquidNFTFactory.result();
    return result;
  }

  //createEmptyLockerJsClient
  createEmptyLocker = async (erc20PackageHash: string) => {
    await this.liquidNFTFactory.setContractHash(this.contractHash!);
    const createEmptyLockerJsClientDeployHash = await this.liquidNFTFactory.createEmptyLockerJsClient(
      KEYS,
      erc20PackageHash!,
      LIQUIDNFT_FACTORY_CREATEEMPTYLOCKER_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... createEmptyLockerJsClient deploy hash: ", createEmptyLockerJsClientDeployHash);

    await getDeploy(NODE_ADDRESS!, createEmptyLockerJsClientDeployHash);
    console.log("... createEmptyLockerJsClient function called successfully");
  }


  //contributeToLocker
  contributeToLocker = async (lockerPackageHash: string, paymentAmount: string) => {
    await this.liquidNFTFactory.setContractHash(this.contractHash!);
    const contributeToLockerJsClientDeployHash = await this.liquidNFTFactory.contributeToLockerJsClient(
      KEYS,
      lockerPackageHash!,
      paymentAmount!,
      LIQUIDNFT_FACTORY_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... contributeToLockerJsClient deploy hash: ", contributeToLockerJsClientDeployHash);

    await getDeploy(NODE_ADDRESS!, contributeToLockerJsClientDeployHash);
    console.log("... contributeToLocker function called successfully");
  }


  //donateToLocker
  donateToLocker = async (lockerPackageHash: string, donationAmount: string) => {
    await this.liquidNFTFactory.setContractHash(this.contractHash!);
    const donateToLockerDeployHash = await this.liquidNFTFactory.donateToLocker(
      KEYS,
      lockerPackageHash!,
      donationAmount!,
      LIQUIDNFT_FACTORY_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... donateToLocker deploy hash: ", donateToLockerDeployHash);

    await getDeploy(NODE_ADDRESS!, donateToLockerDeployHash);
    console.log("... donateToLocker function called successfully");
  }


  //paybackToLocker
  paybackToLocker = async (lockerPackageHash: string, paymentAmount: string) => {
    await this.liquidNFTFactory.setContractHash(this.contractHash!);
    const paybackToLockerDeployHash = await this.liquidNFTFactory.paybackToLocker(
      KEYS,
      lockerPackageHash!,
      paymentAmount!,
      LIQUIDNFT_FACTORY_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... paybackToLocker deploy hash: ", paybackToLockerDeployHash);

    await getDeploy(NODE_ADDRESS!, paybackToLockerDeployHash);
    console.log("... paybackToLocker function called successfully");
  }

  //updateMaster
  updateMaster = async (newMaster: string) => {
    await this.liquidNFTFactory.setContractHash(this.contractHash!);
    const updateMasterDeployHash = await this.liquidNFTFactory.updateMaster(
      KEYS,
      newMaster!,
      LIQUIDNFT_FACTORY_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... updateMaster deploy hash: ", updateMasterDeployHash);

    await getDeploy(NODE_ADDRESS!, updateMasterDeployHash);
    console.log("... updateMaster function called successfully");
  }

  //revokeMaster
  revokeMaster = async () => {
    await this.liquidNFTFactory.setContractHash(this.contractHash!);
    const revokeMasterDeployHash = await this.liquidNFTFactory.revokeMaster(
      KEYS,
      LIQUIDNFT_FACTORY_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... revokeMaster deploy hash: ", revokeMasterDeployHash);

    await getDeploy(NODE_ADDRESS!, revokeMasterDeployHash);
    console.log("... revokeMaster function called successfully");
  }

}

export { LiquidNFTFactory };
