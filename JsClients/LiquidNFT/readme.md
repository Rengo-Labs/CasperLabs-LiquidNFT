# CasperLabs-LiquidNFT-jsClient

This repo has the script code to deploy liquidNFT contract Functions

## Requirement

Make sure you have created and funded the keys before testing.

## Commands

1. Run ```make all`` command.
2. Run ```npm ci``` command in JsClient Folder to install the node packages.
3. if you encounter this error then try again 
4. Must have atleast 500 CSPR for the deployment.
  ```
  type: 'system',
  errno: 'ETIMEDOUT',
  code: 'ETIMEDOUT'
  ```
5. On

Use the script file in package.json to perform the testing
```
"scripts": {
    "calculateEpoch": "ts-node LiquidNFT/scripts/calculateEpoch.ts",
    "calculatePaybacks": "ts-node LiquidNFT/scripts/calculatePaybacks.ts",
    "claimInterestPublic": "ts-node LiquidNFT/scripts/claimInterestPublic.ts",
    "claimInterestSingle": "ts-node LiquidNFT/scripts/claimInterestSingle.ts",
    "rescueLocker": "ts-node LiquidNFT/scripts/rescueLocker.ts",
    "disableLocker": "ts-node LiquidNFT/scripts/disableLocker.ts",
    "donateFunds": "ts-node LiquidNFT/scripts/donateFunds.ts",
    "enableLocker": "ts-node LiquidNFT/scripts/enableLocker.ts",
    "getLateDays": "ts-node LiquidNFT/scripts/getLateDays.ts",
    "increasePaymentRate": "ts-node LiquidNFT/scripts/increasePaymentRate.ts",
    "decreasePaymentRate": "ts-node LiquidNFT/scripts/decreasePaymentRate.ts",
    "initialize": "ts-node LiquidNFT/scripts/initialize.ts",
    "makeContribution": "ts-node LiquidNFT/scripts/makeContribution.ts",
    "liquidateLocker": "ts-node LiquidNFT/scripts/liquidateLocker.ts",
    "payBackFunds": "ts-node LiquidNFT/scripts/payBackFunds.ts",
    "penaltyAmount": "ts-node LiquidNFT/scripts/penaltyAmount.ts",
    "refundDueDisabled": "ts-node LiquidNFT/scripts/refundDueDisabled.ts",
    "refundDueSingle": "ts-node LiquidNFT/scripts/refundDueSingle.ts"
  },
```

Use the following commands to perform testing
```
npm run deployContract
npm run calculateEpoch
npm run calculatePaybacks
npm run claimInterestPublic
npm run claimInterestSingle
npm run rescueLocker
npm run disableLocker
npm run donateFunds
npm run enableLocker
npm run getLateDays
npm run increasePaymentRate
npm run decreasePaymentRate
npm run initialize
npm run makeContribution
npm run liquidateLocker
npm run payBackFunds
npm run penaltyAmount
npm run refundDueDisabled
npm run refundDueSingle

```

CONFIGURE .env BEFORE TESTING

#### Note: .env file is in JSClient folder

go to js client folder
run command npm ci
Copy keys folder to Liquid NFT folder OR generate key using keygen(if using keygen funds account)
