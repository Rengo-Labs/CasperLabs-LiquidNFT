import { config } from "dotenv";
config();
import { LIQUIDNFTClient ,utils, constants} from "../src";
import { sleep, getDeploy } from "./utils";

import {
  CLValueBuilder,
  Keys,
  CLPublicKey,
  CLAccountHash,
  CLPublicKeyType,
  Contracts,
  CLByteArray
} from "casper-js-sdk";

const { LIQUIDNFTEvents } = constants;

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
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

  // // //initialize
  // const initializeDeployHash = await liquidNFT.initialize(
  //   KEYS,
  //   TOKENID!,
  //   TOKENADDRESS!,
  //   TOKENOWNER!,
  //   FLOORASKED!,
  //   TOTALASKED!,
  //   PAYMENTTIME!,
  //   PAYMENTRATE!,
  //   LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
  // );
  // console.log("... Initialize deploy hash: ", initializeDeployHash);

  // await getDeploy(NODE_ADDRESS!, initializeDeployHash);
  // console.log("... Initialize function called successfully");

// //increasePaymentRate
//   const increasePaymentRateDeployHash = await liquidNFT.increasePaymentRate(
//     KEYS,
//     NEWPAYMENTRATE!,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... IncreasePaymentRate deploy hash: ", increasePaymentRateDeployHash);

//   await getDeploy(NODE_ADDRESS!, increasePaymentRateDeployHash);
//   console.log("... IncreasePaymentRate function called successfully");


//   //decreasePaymentTime
//   const decreasePaymentTimeDeployHash = await liquidNFT.decreasePaymentTime(
//     KEYS,
//     NEWPAYMENTRATE!,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... DecreasePaymentTime deploy hash: ", decreasePaymentTimeDeployHash);

//   await getDeploy(NODE_ADDRESS!, decreasePaymentTimeDeployHash);
//   console.log("... DecreasePaymentTime function called successfully");


// //enableLocker
//   const enableLockerDeployHash = await liquidNFT.enableLocker(
//     KEYS,
//     PREPAYAMOUNT!,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... EnableLocker deploy hash: ", enableLockerDeployHash);

//   await getDeploy(NODE_ADDRESS!, enableLockerDeployHash);
//   console.log("... EnableLocker function called successfully");


//   //disableLocker
//   const disableLockerDeployHash = await liquidNFT.disableLocker(
//     KEYS,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... DisableLocker deploy hash: ", disableLockerDeployHash);

//   await getDeploy(NODE_ADDRESS!, disableLockerDeployHash);
//   console.log("... DisableLocker function called successfully");


//   //rescueLocker
//   const rescueLockerDeployHash = await liquidNFT.rescueLocker(
//     KEYS,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... RescueLocker deploy hash: ", rescueLockerDeployHash);

//   await getDeploy(NODE_ADDRESS!, rescueLockerDeployHash);
//   console.log("... RescueLocker function called successfully");


//   //refundDueDisabled
//   const refundDueDisabledDeployHash = await liquidNFT.refundDueDisabled(
//     KEYS,
//     REFUNDADDRESS!,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... RefundDueDisabled deploy hash: ", refundDueDisabledDeployHash);

//   await getDeploy(NODE_ADDRESS!, refundDueDisabledDeployHash);
//   console.log("... RefundDueDisabled function called successfully");


//   //refundDueSingle
//   const refundDueSingleDeployHash = await liquidNFT.refundDueSingle(
//     KEYS,
//     REFUNDADDRESS!,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... RefundDueSingle deploy hash: ", refundDueSingleDeployHash);

//   await getDeploy(NODE_ADDRESS!, refundDueSingleDeployHash);
//   console.log("... RefundDueSingle function called successfully");


//   //donateFunds
//   const donateFundsDeployHash = await liquidNFT.donateFunds(
//     KEYS,
//     DONATIONAMOUNT!,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... DonateFunds deploy hash: ", donateFundsDeployHash);

//   await getDeploy(NODE_ADDRESS!, donateFundsDeployHash);
//   console.log("... DonateFunds function called successfully");


//   //payBackFunds
//   const payBackFundsDeployHash = await liquidNFT.payBackFunds(
//     KEYS,
//     PAYMENTAMOUNTARGS!,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... PayBackFunds deploy hash: ", payBackFundsDeployHash);

//   await getDeploy(NODE_ADDRESS!, payBackFundsDeployHash);
//   console.log("... PayBackFunds function called successfully");


//   //liquidateLocker
//   const liquidateLockerDeployHash = await liquidNFT.liquidateLocker(
//     KEYS,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... LiquidateLocker deploy hash: ", liquidateLockerDeployHash);

//   await getDeploy(NODE_ADDRESS!, payBackFundsDeployHash);
//   console.log("... LiquidateLocker function called successfully");


//   //claimInterestSingle
//   const claimInterestSingleDeployHash = await liquidNFT.claimInterestSingle(
//     KEYS,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... ClaimInterestSingle deploy hash: ", claimInterestSingleDeployHash);

//   await getDeploy(NODE_ADDRESS!, claimInterestSingleDeployHash);
//   console.log("... ClaimInterestSingle function called successfully");


//   //claimInterestPublic
//   const claimInterestPublicDeployHash = await liquidNFT.claimInterestPublic(
//     KEYS,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... ClaimInterestPublic deploy hash: ", claimInterestPublicDeployHash);

//   await getDeploy(NODE_ADDRESS!, claimInterestPublicDeployHash);
//   console.log("... ClaimInterestPublic function called successfully");


//   //calculateEpoch
//   const calculateEpochDeployHash = await liquidNFT.calculateEpoch(
//     KEYS,
//     TOTALVALUE!,
//     PAYMENTTIME!,
//     PAYMENTRATE!,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... CalculateEpoch deploy hash: ", calculateEpochDeployHash);

//   await getDeploy(NODE_ADDRESS!, calculateEpochDeployHash);
//   console.log("... CalculateEpoch function called successfully");


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
//   const getLateDaysDeployHash = await liquidNFT.getLateDays(
//     KEYS,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... GetLateDays deploy hash: ", getLateDaysDeployHash);

//   await getDeploy(NODE_ADDRESS!, getLateDaysDeployHash);
//   console.log("... GetLateDays function called successfully");


//   //penaltyAmount
//   const penaltyAmountDeployHash = await liquidNFT.penaltyAmount(
//     KEYS,
//     TOTALCOLLECTED!,
//     LATEDAYSAMOUNT!,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... PenaltyAmount deploy hash: ", penaltyAmountDeployHash);

//   await getDeploy(NODE_ADDRESS!, penaltyAmountDeployHash);
//   console.log("... PenaltyAmount function called successfully");


//   //makeContribution
//   const makeContributionDeployHash = await liquidNFT.makeContribution(
//     KEYS,
//     TOKENAMOUNT!,
//     TOKENHOLDER!,
//     LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
//   );
//   console.log("... MakeContribution deploy hash: ", makeContributionDeployHash);

//   await getDeploy(NODE_ADDRESS!, makeContributionDeployHash);
//   console.log("... MakeContribution function called successfully");


};

test();

