# CasperLabs-LiquidNFT-jsClient

This folder has the test code to do testing of liquidNFT Factory flows

## Requirement

Make sure you have created and funded the keys before testing.
Copy keys folder to liquidNFT-Factory-Tests-Scripts folder

Use the script file in package.json to perform the testing
```
"scripts": {
    "testTooLatePayBackFlow": "ts-node script/testTooLatePayBackFlow.ts"
  },
```

Use the following commands to perform testing
```
npm run testTooLatePayBackFlow  <id> <key> <value> <liquidNFTFactoryPackageHash> <cep47PackageHash> <floorAsked> <totalFloor> <paymentTime> <paymentRate> <erc20PackageHash> <keyPath> <mintApproveAmount> <paymentAmountContributeToLocker> <prePaymentAmount> <paymentAmountForPaybackToLocker> 

```

## Flow of should be able to liquadateLocker when payment is late: 

1) Mint NFT against Onwer in cep47 JsClient using mintOneToken function

2) Approve NFT against LiquidNFT Factory Package Hash in cep47 JsClient using approveOneToken function

3) Lock NFT in the locker using funtion createliquidLockerJsClient

IMPORTANT NOTE: paymentTime and paymentRate values should be in proportion to each other
(Neither too big nor too low, else you will get user errors in other functions)

4) Mint erc20 tokens against Onwer in erc20 JsClient using mint function (which you want to contribute)

5) Approve erc20 tokens against LiquidNFT Factory Package Hash in erc20 JsClient using approve
function(which you want to contribute)

6) Contribute to the locker using contributeToLocker function

7) Enable the locker by calling enableLocker function using LiquidNFT JsClient
IMPORTANT NOTE: Make sure floor asked has reached

8) wait 20 minutes

9) Call payBackToLocker function, you will receive user error: 141 (TooLate)

10) Call liquidateLocker function 


## DeployHashes For ToLatePayment liquidate Flow: 

```
mintOneToken: 400d49165137ac023c5017a885ad007006d173362394e65cc37daa3b600ba635
approveOneToken: 05a9e3061b47538f5bf4b3bd7eab6cb73839995ed86e38fed0f5d0b9fa6b3d8d
createLiquidLockerJsClient: c4fb65891dbf4f399baaa058327340740f9baf93e87cbf72bfd0089c9060008a
mint: 51652aaaae3fc6ca42008a1af81afabef9d204340fab4fba4c6695ccd527bd1b
approve: ed24d4231092b6dae7d4ca98e106c1483133a7017e1ecb7a6690bf52b7bab115
contributeToLocker: 0532d8b5ffe8d11b9662fc3cd5bcda98d5180a417e9eeb17d968720121220e4c
enableLocker: 297a06b50a5cbfd03cf13593e9bb433e1da35344f1be40a900a3a12842893a97
paybackToLocker: f2bdcb3fd6355d61c70df7bbaae85bfa3b074653478aa389e30b0464c1aa8b9f
liquidateLocker: 1d600938f5d3f8e2c047f6dfea7ce14b08176cc7c4cc5d87c18e9ac35e9bf742

```