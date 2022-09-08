# CasperLabs-LiquidNFT-jsClient

This folder has the fs filesystem code to read wasm and deploy liquidNFT Factory contract and its functions

## Requirement

Make sure you have created and funded the keys before testing.

## Commands

1. Run ```make all`` command.
2. Run ```npm ci``` command in JsClient Folder to install the node packages.
3. Run ```npm run deployContract``` to deploy contracts to test-net.
4. if you encounter this error then try again 
5. Must have atleast 500 CSPR for the deployment.
  ```
  type: 'system',
  errno: 'ETIMEDOUT',
  code: 'ETIMEDOUT'
  ```
5. On

Use the script file in package.json to perform the testing
```
"scripts": {
    "deployContract": "ts-node LiquidNFT-Factory/scripts/deployContract.ts",
    "createLiquidLocker": "ts-node LiquidNFT-Factory/scripts/createLiquidLocker.ts",
    "createEmptyLocker": "ts-node LiquidNFT-Factory/scripts/createEmptyLocker.ts",
    "contributeToLocker": "ts-node LiquidNFT-Factory/scripts/contributeToLocker.ts",
    "donateToLocker": "ts-node LiquidNFT-Factory/scripts/donateToLocker.ts",
    "updateMaster": "ts-node LiquidNFT-Factory/scripts/updateMaster.ts",
    "revokeMaster": "ts-node LiquidNFT-Factory/scripts/revokeMaster.ts",
    "lockerHashes": "ts-node LiquidNFT-Factory/scripts/lockerHashes.ts",
    "testMainContractFlow": "ts-node LiquidNFT-Factory/scripts/testMainContractFlow.ts"
  },
```

Use the following commands to perform testing
```
npm run deployContract <defaultToken>
npm run createLiquidLocker <tokenId> <cep47PackageHash> <floorAsked> <totalAsked> <paymentTime> <paymentRate> <erc20PackageHash>
npm run createEmptyLocker <erc20PackageHash>
npm run contributeToLocker <lockerPackageHash> <paymentAmount>
npm run paybackToLocker <lockerPackageHash> <paymentAmount>
npm run donateToLocker <lockerPackageHash> <donationAmount>
npm run updateMaster <newMasterAccountHash>
npm run revokeMaster
npm run lockerHashes
npm run testMainContractFlow  <id> <key> <value> <liquidNFTFactoryPackageHash> <cep47PackageHash> <floorAsked> <totalFloor> <paymentTime> <paymentRate> <erc20PackageHash> <keyPath> <mintApproveAmount> <paymentAmountContributeToLocker> <prePaymentAmount> <paymentAmountForPaybackToLocker> 

```

CONFIGURE .env BEFORE TESTING

#### Note: .env file is in JSClient folder

go to js client folder
run command npm ci
Copy keys folder to Liquid NFT folder OR generate key using keygen(if using keygen funds account)


## Main Flow of the factory contract 

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

8) Call payBackToLocker function 


## Successfull DeployHashes: 

```
mintOneToken: a95b2e1ebff578dad966f69e0524462f14da8f0fdfdbf45b40aebbce825ab18e
approveOneToken: 3ace7cb5d2211ea3d82be8a71ceb0453f60eeec959b51a30fa47caba61d49d80
createLiquidLockerJsClient: 437c526d50524ca412b36578033948d141d30af3be9ef3ffe1bbead477228692
mint: 7f59b10de96cf5a2b8b298666cf301766fed3b9aeb965baf2ebbf9bccfd1f8d7
approve: c2d60eef9ec20733619ed492132758abc7c6609722cab698a36c65440dcbabb8
contributeToLocker: c1dcf343f2c5c392e898a4a49ec4d4c93b68ae1310070b71a45de6ca36e69b37
enableLocker: b34ab4cc4cf3745f55ce05e0da491020c317bb1468ecbb49abe40aa5b8ac9956
paybackToLocker: e610daa74f2591b34c30f44a8ee6e41d15cff06d2762757cbbbcc81939401162

```