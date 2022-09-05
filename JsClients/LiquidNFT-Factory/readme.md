# CasperLabs-LiquidNFT-jsClient

This repo has the fs filesystem code to read wasm and deploy liquidNFT Factory contract and its functions

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
    "lockerHashes": "ts-node LiquidNFT-Factory/scripts/lockerHashes.ts"
  },
```

Use the following commands to perform testing
```
npm run deployContract
npm run createLiquidLocker
npm run createEmptyLocker
npm run contributeToLocker
npm run donateToLocker
npm run updateMaster
npm run revokeMaster
npm run lockerHashes

```

CONFIGURE .env BEFORE TESTING

#### Note: .env file is in JSClient folder

go to js client folder
run command npm ci
Copy keys folder to Liquid NFT folder OR generate key using keygen(if using keygen funds account)
