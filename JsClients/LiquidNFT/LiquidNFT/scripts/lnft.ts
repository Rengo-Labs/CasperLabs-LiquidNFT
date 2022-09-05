import { config } from "dotenv";
config();
import LIQUIDNFTClientForFunctions from "../../../../JsClientsForFrontend/LiquidNFT/src/liquidNFT";
import { getDeploy } from "./utils";

import {
  Keys,
} from "casper-js-sdk";

import * as fs from 'fs';

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  LIQUIDNFT_MASTER_KEY_PAIR_PATH,
  LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT,
} = process.env;


const KEYS = Keys.Ed25519.parseKeyFiles(
  `${LIQUIDNFT_MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${LIQUIDNFT_MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

class LiquidNFT {
  contractHash: string;
  liquidNFT: LIQUIDNFTClientForFunctions;
  constructor() {
    let _contractHash = fs.readFileSync('contractHash','utf8');
    this.contractHash = _contractHash.split("-").pop()!;
    this.liquidNFT = new LIQUIDNFTClientForFunctions(
      NODE_ADDRESS!,
      CHAIN_NAME!,
      EVENT_STREAM_ADDRESS!
    );
  }
  
  //initialize
  initialize = async(
    tokenIdsArray: Array<string>,
    cep47PackageHash: string,
    tokenOwner: string,
    floorAsked:string,
    totalAsked: string,
    paymentTime: string,
    paymentRate: string,
  )=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const initializeDeployHash = await this.liquidNFT.initialize(
      KEYS,
      tokenIdsArray!,
      cep47PackageHash!,
      tokenOwner!,
      floorAsked!,
      totalAsked!,
      paymentTime!,
      paymentRate!,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... initialize deploy hash: ", initializeDeployHash);

    await getDeploy(NODE_ADDRESS!, initializeDeployHash);
    console.log("... initialize function called successfully");
  }

  //makeContribution
  makeContribution = async(
    tokenAmount:string,
    tokenHolder: string
  )=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const makeContributionDeployHash = await this.liquidNFT.makeContribution(
      KEYS,
      tokenAmount!,
      tokenHolder!,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... makeContribution deploy hash: ", makeContributionDeployHash);

    await getDeploy(NODE_ADDRESS!, makeContributionDeployHash);
    console.log("... makeContribution function called successfully");
  }

  //increasePaymentRate
  increasePaymentRate = async(
    newPaymentRate:string
  )=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const increasePaymentRateDeployHash = await this.liquidNFT.increasePaymentRate(
      KEYS,
      newPaymentRate!,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... increasePaymentRate deploy hash: ", increasePaymentRateDeployHash);

    await getDeploy(NODE_ADDRESS!, increasePaymentRateDeployHash);
    console.log("... increasePaymentRate function called successfully");
  }

  //decreasePaymentTime
  decreasePaymentTime = async(
    newPaymentRate:string
  )=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const decreasePaymentTimeDeployHash = await this.liquidNFT.decreasePaymentTime(
      KEYS,
      newPaymentRate!,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... decreasePaymentTime deploy hash: ", decreasePaymentTimeDeployHash);

    await getDeploy(NODE_ADDRESS!, decreasePaymentTimeDeployHash);
    console.log("... decreasePaymentTime function called successfully");
  }


  //enableLocker
  enableLocker = async(
    prePaymentAmount:string
  )=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const enableLockerDeployHash = await this.liquidNFT.enableLocker(
      KEYS,
      prePaymentAmount!,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... enableLocker deploy hash: ", enableLockerDeployHash);

    await getDeploy(NODE_ADDRESS!, enableLockerDeployHash);
    console.log("... enableLocker function called successfully");
  }

  //disableLocker
  disableLocker = async()=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const disableLockerDeployHash = await this.liquidNFT.disableLocker(
      KEYS,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("... disableLocker deploy hash: ", disableLockerDeployHash);

    await getDeploy(NODE_ADDRESS!, disableLockerDeployHash);
    console.log("... disableLocker function called successfully");
  }

  //rescueLocker
  rescueLocker = async()=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const rescueLockerDeployHash = await this.liquidNFT.rescueLocker(
      KEYS,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("...rescueLocker deploy hash: ", rescueLockerDeployHash);

    await getDeploy(NODE_ADDRESS!, rescueLockerDeployHash);
    console.log("... rescueLocker function called successfully");
  }

  //liquidateLocker
  liquidateLocker = async()=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const liquidateLockerDeployHash = await this.liquidNFT.liquidateLocker(
      KEYS,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("...liquidateLocker deploy hash: ", liquidateLockerDeployHash);

    await getDeploy(NODE_ADDRESS!, liquidateLockerDeployHash);
    console.log("... liquidateLocker function called successfully");
  }

  //refundDueDisabled
  refundDueDisabled = async(
    refundAddress:string
  )=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const refundDueDisabledDeployHash = await this.liquidNFT.refundDueDisabled(
      KEYS,
      refundAddress!,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("...refundDueDisabled deploy hash: ", refundDueDisabledDeployHash);

    await getDeploy(NODE_ADDRESS!, refundDueDisabledDeployHash);
    console.log("... refundDueDisabled function called successfully");
  }

  //refundDueSingle
  refundDueSingle = async(
    refundAddress:string
  )=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const refundDueSingleDeployHash = await this.liquidNFT.refundDueSingle(
      KEYS,
      refundAddress!,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("...refundDueSingle deploy hash: ", refundDueSingleDeployHash);

    await getDeploy(NODE_ADDRESS!, refundDueSingleDeployHash);
    console.log("... refundDueSingle function called successfully");
  }

  //donateFunds
  donateFunds = async(
    donationAmount:string
  )=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const donateFundsDeployHash = await this.liquidNFT.donateFunds(
      KEYS,
      donationAmount!,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("...donateFunds deploy hash: ", donateFundsDeployHash);

    await getDeploy(NODE_ADDRESS!, donateFundsDeployHash);
    console.log("... donateFunds function called successfully");
  }

  //payBackFunds
  payBackFunds = async(
    paymentAmount:string
  )=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const payBackFundsDeployHash = await this.liquidNFT.payBackFunds(
      KEYS,
      paymentAmount!,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("...payBackFunds deploy hash: ", payBackFundsDeployHash);

    await getDeploy(NODE_ADDRESS!, payBackFundsDeployHash);
    console.log("... payBackFunds function called successfully");
  }

  //claimInterestSingle
  claimInterestSingle = async()=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const claimInterestSingleDeployHash = await this.liquidNFT.claimInterestSingle(
      KEYS,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("...claimInterestSingle deploy hash: ", claimInterestSingleDeployHash);

    await getDeploy(NODE_ADDRESS!, claimInterestSingleDeployHash);
    console.log("... claimInterestSingle function called successfully");
  }

  //claimInterestPublic
  claimInterestPublic = async()=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const claimInterestPublicDeployHash = await this.liquidNFT.claimInterestPublic(
      KEYS,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("...claimInterestPublic deploy hash: ", claimInterestPublicDeployHash);

    await getDeploy(NODE_ADDRESS!, claimInterestPublicDeployHash);
    console.log("... claimInterestPublic function called successfully");
  }

  //calculateEpoch
  calculateEpoch = async(
    totalValue:string,
    paymentRate:string,
    paymentTime:string
  )=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const calculateEpochDeployHash = await this.liquidNFT.calculateEpoch(
      KEYS,
      totalValue!,
      paymentRate!,
      paymentTime!,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("...calculateEpoch deploy hash: ", calculateEpochDeployHash);

    await getDeploy(NODE_ADDRESS!, calculateEpochDeployHash);
    console.log("... calculateEpoch function called successfully");
  }

  //calculatePaybacks
  calculatePaybacks  = async(
    totalValue:string,
    paymentRate:string,
    paymentTime:string
  )=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const calculatePaybacksDeployHash = await this.liquidNFT.calculatePaybacks(
      KEYS,
      totalValue!,
      paymentRate!,
      paymentTime!,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("...calculatePaybacks deploy hash: ", calculatePaybacksDeployHash);

    await getDeploy(NODE_ADDRESS!, calculatePaybacksDeployHash);
    console.log("... calculatePaybacks function called successfully");
  }

  //getLateDays
  getLateDays  = async()=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const getLateDaysDeployHash = await this.liquidNFT.getLateDays(
      KEYS,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("...getLateDays deploy hash: ", getLateDaysDeployHash);

    await getDeploy(NODE_ADDRESS!, getLateDaysDeployHash);
    console.log("... getLateDays function called successfully");
  }

  //penaltyAmount
  penaltyAmount  = async(
    totalCollected:string,
    lateDaysAmount:string
  )=>{
    await this.liquidNFT.setContractHash(this.contractHash!);
    const penaltyAmountDeployHash = await this.liquidNFT.penaltyAmount(
      KEYS,
      totalCollected!,
      lateDaysAmount!,
      LIQUIDNFT_FUNCTIONS_PAYMENT_AMOUNT!
    );
    console.log("...penaltyAmount deploy hash: ", penaltyAmountDeployHash);

    await getDeploy(NODE_ADDRESS!, penaltyAmountDeployHash);
    console.log("... penaltyAmount function called successfully");
  }

}

export{LiquidNFT};





