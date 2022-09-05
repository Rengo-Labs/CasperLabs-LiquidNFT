import { config } from "dotenv";
config();
import { LIQUIDNFTClient ,utils, constants} from "../src";
import { getDeploymentCount,updateDeploymentCount,getKeys,sleep, getDeploy } from "./utils";

import {
  CLValueBuilder,
  Keys,
  CLPublicKey,
  CLAccountHash,
  CLPublicKeyType,
  Contracts,
  CLByteArray
} from "casper-js-sdk";
import * as fs from 'fs';

const { LIQUIDNFTEvents } = constants;

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  LIQUIDNFT_INSTALL_PAYMENT_AMOUNT,
  LIQUIDNFT_WASM_PATH,
  LIQUIDNFT_MASTER_KEY_PAIR_PATH,
  LIQUIDNFT_CONTRACT_NAME,
  LIQUIDNFT_CONTRACT_HASH,
  LIQUIDNFT_PACKAGE_HASH,
  LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT,
  TOKENID,
  TOKENADDRESS,
  TOKENOWNER,
  FLOORASKED,
  TOTALASKED,
  PAYMENTTIME,
  PAYMENTRATE,
  NEWPAYMENTRATE,
  PREPAYAMOUNT,
  REFUNDADDRESS,
  DONATIONAMOUNT,
  PAYMENTAMOUNTARGS,
  TOTALVALUE,
  TOTALCOLLECTED,
  LATEDAYSAMOUNT,
  TOKENAMOUNT,
  TOKENHOLDER,
  PAYMENTTOKEN,
  LOCKERSADDRESS,
} = process.env;


const KEYS = Keys.Ed25519.parseKeyFiles(
  `${LIQUIDNFT_MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${LIQUIDNFT_MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

function splitdata(data:string)
{
    var temp=data.split('(');
    var result=temp[1].split(')');
    return result[0];
}

const liquidNFT = new LIQUIDNFTClient(
  NODE_ADDRESS!,
  CHAIN_NAME!,
  EVENT_STREAM_ADDRESS!
);

const test = async () => {

  await liquidNFT.setContractHash(LIQUIDNFT_CONTRACT_HASH!);
 // await liquidNFT.setContractHash('e64594d4b422533759416a02a489decab37a214bed2768ebb2bab8fdd63f0071');

  //    const result = await liquidNFT.result();
  //  console.log(`... Contract name: ${result}`);

  // const result = await liquidNFT.result();
  // console.log(`... Result Key Value: ${result}`);

//FACTORY FUNCTIONS

//updateMaster
  // const updateMasterDeployHash = await liquidNFT.updateMaster(
  //   KEYS,
  //   ///NEWMASTER!,
  //   KEYS.publicKey,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... updateMaster deploy hash: ", updateMasterDeployHash);

  // await getDeploy(NODE_ADDRESS!, updateMasterDeployHash);
  // console.log("... updateMaster function called successfully");


  //revokeMaster
  // const revokeMasterDeployHash = await liquidNFT.revokeMaster(
  //   KEYS,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... revokeMaster deploy hash: ", revokeMasterDeployHash);

  // await getDeploy(NODE_ADDRESS!, revokeMasterDeployHash);
  // console.log("... revokeMaster function called successfully");


  //   // // //createLiquidLocker
  // const createLiquidLockerJsClientDeployHash = await liquidNFT.createLiquidLockerJsClient(
  //   KEYS,
  //   ["68","78"],
  //   TOKENADDRESS!,
  //   //KEYS.publicKey!,
  //   FLOORASKED!,
  //   TOTALASKED!,
  //   PAYMENTTIME!,
  //   PAYMENTRATE!,
  //   PAYMENTTOKEN!,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... createLiquidLockerJsClient deploy hash: ", createLiquidLockerJsClientDeployHash);

  // await getDeploy(NODE_ADDRESS!, createLiquidLockerJsClientDeployHash);
  // console.log("... createLiquidLockerJsClient function called successfully");


  //createEmptyLockerJsClient
  // const createEmptyLockerJsClientDeployHash = await liquidNFT.createEmptyLockerJsClient(
  //   KEYS,
  //   PAYMENTTOKEN!,
  //   //KEYS.publicKey,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... createEmptyLockerJsClient deploy hash: ", createEmptyLockerJsClientDeployHash);

  // await getDeploy(NODE_ADDRESS!, createEmptyLockerJsClientDeployHash);
  // console.log("... createEmptyLockerJsClient function called successfully");


  //contributeToLocker
  // const contributeToLockerJsClientDeployHash = await liquidNFT.contributeToLockerJsClient(
  //   KEYS,
  //   LOCKERSADDRESS!,
  //   //KEYS.publicKey,
  //   PAYMENTAMOUNTARGS,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... contributeToLockerJsClient deploy hash: ", contributeToLockerJsClientDeployHash);
  
  // await getDeploy(NODE_ADDRESS!, contributeToLockerJsClientDeployHash);
  // console.log("... contributeToLocker function called successfully");


  //donateToLocker
  // const donateToLockerDeployHash = await liquidNFT.donateToLocker(
  //   KEYS,
  //   LOCKERSADDRESS!,
  //   //KEYS.publicKey,
  //   DONATIONAMOUNT,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... donateToLocker deploy hash: ", donateToLockerDeployHash);
  
  // await getDeploy(NODE_ADDRESS!, donateToLockerDeployHash);
  // console.log("... donateToLocker function called successfully");  


  //paybackToLocker
  const paybackToLockerDeployHash = await liquidNFT.paybackToLocker(
    KEYS,
    LOCKERSADDRESS!,
    //KEYS.publicKey,
    PAYMENTAMOUNTARGS!,
    LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... paybackToLocker deploy hash: ", paybackToLockerDeployHash);
  
  await getDeploy(NODE_ADDRESS!, paybackToLockerDeployHash);
  console.log("... paybackToLocker function called successfully");



//   // // //initialize
  // const initializeDeployHash = await liquidNFT.initialize(
  //   KEYS,
  //   ["67","77"],
  //   TOKENADDRESS!,
  //   TOKENOWNER!,
  //   //KEYS.publicKey!,
  //   FLOORASKED!,
  //   TOTALASKED!,
  //   PAYMENTTIME!,
  //   PAYMENTRATE!,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... Initialize deploy hash: ", initializeDeployHash);

  // await getDeploy(NODE_ADDRESS!, initializeDeployHash);
  // console.log("... Initialize function called successfully");


  
//   //   //makeContribution
  // const makeContributionDeployHash = await liquidNFT.makeContribution(
  //   KEYS,
  //   TOKENAMOUNT!,
  //   //KEYS.publicKey,
  //   TOKENHOLDER!,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... MakeContribution deploy hash: ", makeContributionDeployHash);

  // await getDeploy(NODE_ADDRESS!, makeContributionDeployHash);
  // console.log("... MakeContribution function called successfully");

//increasePaymentRate
  // const increasePaymentRateDeployHash = await liquidNFT.increasePaymentRate(
  //   KEYS,
  //   NEWPAYMENTRATE!,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... IncreasePaymentRate deploy hash: ", increasePaymentRateDeployHash);

  // await getDeploy(NODE_ADDRESS!, increasePaymentRateDeployHash);
  // console.log("... IncreasePaymentRate function called successfully");


//   //decreasePaymentTime
  // const decreasePaymentTimeDeployHash = await liquidNFT.decreasePaymentTime(
  //   KEYS,
  //   NEWPAYMENTRATE!,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... DecreasePaymentTime deploy hash: ", decreasePaymentTimeDeployHash);

  // await getDeploy(NODE_ADDRESS!, decreasePaymentTimeDeployHash);
  // console.log("... DecreasePaymentTime function called successfully");


// // //enableLocker
  // const enableLockerDeployHash = await liquidNFT.enableLocker(
  //   KEYS,
  //   PREPAYAMOUNT!,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... EnableLocker deploy hash: ", enableLockerDeployHash);

  // await getDeploy(NODE_ADDRESS!, enableLockerDeployHash);
  // console.log("... EnableLocker function called successfully");


  //disableLocker
  // const disableLockerDeployHash = await liquidNFT.disableLocker(
  //   KEYS,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... DisableLocker deploy hash: ", disableLockerDeployHash);

  // await getDeploy(NODE_ADDRESS!, disableLockerDeployHash);
  // console.log("... DisableLocker function called successfully");


//   //rescueLocker
  // const rescueLockerDeployHash = await liquidNFT.rescueLocker(
  //   KEYS,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... RescueLocker deploy hash: ", rescueLockerDeployHash);

  // await getDeploy(NODE_ADDRESS!, rescueLockerDeployHash);
  // console.log("... RescueLocker function called successfully");


//   //refundDueDisabled
  // const refundDueDisabledDeployHash = await liquidNFT.refundDueDisabled(
  //   KEYS,
  //   //REFUNDADDRESS!,
  //   KEYS.publicKey,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... RefundDueDisabled deploy hash: ", refundDueDisabledDeployHash);

  // await getDeploy(NODE_ADDRESS!, refundDueDisabledDeployHash);
  // console.log("... RefundDueDisabled function called successfully");


//   //refundDueSingle
//   const refundDueSingleDeployHash = await liquidNFT.refundDueSingle(
//     KEYS,
//     REFUNDADDRESS!,
//     //KEYS.publicKey,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... RefundDueSingle deploy hash: ", refundDueSingleDeployHash);

//   await getDeploy(NODE_ADDRESS!, refundDueSingleDeployHash);
//   console.log("... RefundDueSingle function called successfully");


//   //donateFunds
  // const donateFundsDeployHash = await liquidNFT.donateFunds(
  //   KEYS,
  //   DONATIONAMOUNT!,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... DonateFunds deploy hash: ", donateFundsDeployHash);

  // await getDeploy(NODE_ADDRESS!, donateFundsDeployHash);
  // console.log("... DonateFunds function called successfully");


// //   //payBackFunds
  // const payBackFundsDeployHash = await liquidNFT.payBackFunds(
  //   KEYS,
  //   PAYMENTAMOUNTARGS!,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... PayBackFunds deploy hash: ", payBackFundsDeployHash);

  // await getDeploy(NODE_ADDRESS!, payBackFundsDeployHash);
  // console.log("... PayBackFunds function called successfully");


//   //liquidateLocker
  // const liquidateLockerDeployHash = await liquidNFT.liquidateLocker(
  //   KEYS,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... LiquidateLocker deploy hash: ", liquidateLockerDeployHash);

  // await getDeploy(NODE_ADDRESS!, liquidateLockerDeployHash);
  // console.log("... LiquidateLocker function called successfully");


//   //claimInterestSingle
  // const claimInterestSingleDeployHash = await liquidNFT.claimInterestSingle(
  //   KEYS,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... ClaimInterestSingle deploy hash: ", claimInterestSingleDeployHash);

  // await getDeploy(NODE_ADDRESS!, claimInterestSingleDeployHash);
  // console.log("... ClaimInterestSingle function called successfully");


//   //claimInterestPublic
  // const claimInterestPublicDeployHash = await liquidNFT.claimInterestPublic(
  //   KEYS,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... ClaimInterestPublic deploy hash: ", claimInterestPublicDeployHash);

  // await getDeploy(NODE_ADDRESS!, claimInterestPublicDeployHash);
  // console.log("... ClaimInterestPublic function called successfully");


//   //calculateEpoch
  // const calculateEpochDeployHash = await liquidNFT.calculateEpoch(
  //   KEYS,
  //   TOTALVALUE!,
  //   PAYMENTTIME!,
  //   PAYMENTRATE!,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... CalculateEpoch deploy hash: ", calculateEpochDeployHash);

  // await getDeploy(NODE_ADDRESS!, calculateEpochDeployHash);
  // console.log("... CalculateEpoch function called successfully");


//   //calculatePaybacks
//   const calculatePaybacksDeployHash = await liquidNFT.calculatePaybacks(
//     KEYS,
//     TOTALVALUE!,
//     PAYMENTTIME!,
//     PAYMENTRATE!,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... calculatePaybacks deploy hash: ", calculatePaybacksDeployHash);

//   await getDeploy(NODE_ADDRESS!, calculatePaybacksDeployHash);
//   console.log("... calculatePaybacks function called successfully");


//   //getLateDays
  // const getLateDaysDeployHash = await liquidNFT.getLateDays(
  //   KEYS,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... GetLateDays deploy hash: ", getLateDaysDeployHash);

  // await getDeploy(NODE_ADDRESS!, getLateDaysDeployHash);
  // console.log("... GetLateDays function called successfully");


//   //penaltyAmount
  // const penaltyAmountDeployHash = await liquidNFT.penaltyAmount(
  //   KEYS,
  //   TOTALCOLLECTED!,
  //   LATEDAYSAMOUNT!,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... PenaltyAmount deploy hash: ", penaltyAmountDeployHash);

  // await getDeploy(NODE_ADDRESS!, penaltyAmountDeployHash);
  // console.log("... PenaltyAmount function called successfully");


//   // //makeContribution
//   // const makeContributionDeployHash = await liquidNFT.makeContribution(
//   //   KEYS,
//   //   TOKENAMOUNT!,
//   //   KEYS.publicKey,
//   //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   // );
//   // console.log("... MakeContribution deploy hash: ", makeContributionDeployHash);

//   // await getDeploy(NODE_ADDRESS!, makeContributionDeployHash);
//   // console.log("... MakeContribution function called successfully");


};

test();

class LiquidNFT {
  contractHash: string;
  liquidNFT: LIQUIDNFTClient;
  Ready: Promise<any>;
  constructor() {
    let _contractHash = fs.readFileSync('contractHash','utf8');
    this.contractHash = _contractHash.split("-").pop()!;

    this.liquidNFT = new LIQUIDNFTClient(
      NODE_ADDRESS!,
      CHAIN_NAME!,
      EVENT_STREAM_ADDRESS!
    );
  }
  
  
  /*
@dev
params: default token
provide the payment tokenERC20 package hash
*/

deployContract = async (defaultToken: string) => {
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

  
  //   // // //createLiquidLocker
  createLiquidLockerJsClient = async(
    tokenIdsArray: Array<string>,
    cep47PackageHash: string,
    floorAsked: string,
    totalAsked: string,
    paymentTime: string,
    paymentRate: string,
    erc20PackageHash: string,
  )=>{
  const createLiquidLockerJsClientDeployHash = await liquidNFT.createLiquidLockerJsClient(
    KEYS,
    tokenIdsArray!,
    cep47PackageHash!,
    floorAsked!,
    totalAsked!,
    paymentTime!,
    paymentRate!,
    erc20PackageHash!,
    LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... createLiquidLockerJsClient deploy hash: ", createLiquidLockerJsClientDeployHash);

  await getDeploy(NODE_ADDRESS!, createLiquidLockerJsClientDeployHash);
  console.log("... createLiquidLockerJsClient function called successfully");
  }


  //createEmptyLockerJsClient
  createEmptyLocker =async (erc20PackageHash: string) => {
  const createEmptyLockerJsClientDeployHash = await liquidNFT.createEmptyLockerJsClient(
    KEYS,
    erc20PackageHash!,
    LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... createEmptyLockerJsClient deploy hash: ", createEmptyLockerJsClientDeployHash);

  await getDeploy(NODE_ADDRESS!, createEmptyLockerJsClientDeployHash);
  console.log("... createEmptyLockerJsClient function called successfully");
  }


  //contributeToLocker
  contributeToLocker = async (lockerPackageHash: string, paymentAmount: string) => {
  const contributeToLockerJsClientDeployHash = await liquidNFT.contributeToLockerJsClient(
    KEYS,
    lockerPackageHash!,
    paymentAmount!,
    LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... contributeToLockerJsClient deploy hash: ", contributeToLockerJsClientDeployHash);
  
  await getDeploy(NODE_ADDRESS!, contributeToLockerJsClientDeployHash);
  console.log("... contributeToLocker function called successfully");
  }


  //donateToLocker
  donateToLocker =async (lockerPackageHash: string, donationAmount:string) => {
  const donateToLockerDeployHash = await liquidNFT.donateToLocker(
    KEYS,
    lockerPackageHash!,
    donationAmount!,
    LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... donateToLocker deploy hash: ", donateToLockerDeployHash);
  
  await getDeploy(NODE_ADDRESS!, donateToLockerDeployHash);
  console.log("... donateToLocker function called successfully");  
  }





  //paybackToLocker
  paybackToLocker =async (lockerPackageHash:string,paymentAmount:string ) => {
  const paybackToLockerDeployHash = await liquidNFT.paybackToLocker(
    KEYS,
    lockerPackageHash!,
    paymentAmount!,
    LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  );
  console.log("... paybackToLocker deploy hash: ", paybackToLockerDeployHash);
  
  await getDeploy(NODE_ADDRESS!, paybackToLockerDeployHash);
  console.log("... paybackToLocker function called successfully");
  }

  lockerHashes = async() => {
    const result = await liquidNFT.result();
  console.log(`... Result Key Value: ${result}`);
  }


}

export{LiquidNFT};