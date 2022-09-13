# CasperLabs-LiquidNFT-JsClients

This folder have cep47, erc20 Jsclients and scripts for LiquidNFT-Factory and LiquidNFT JsClients

## Prequisites

1) Go to JsClientsForFrontend/LiquidNFT folder and npm i in it.

2) Go to JsClientsForFrontend/LiquidNFT-Factory folder and npm i in it.

3) Go to casper-cep47 folder and npm i in it.

4) Go to LiquidNFT folder and npm i in it.

5) Go to LiquidNFT-Factory folder and npm i in it.

6) Go to uniswapV2Core-erc20 folder and npm i in it.

7) Go to LiquidNFT-Factory-Tests-Scripts/mainContractFlowScript folder and npm i in it.

8) Go to LiquidNFT-Factory-Tests-Scripts/tooLateLiquidateFlowScript folder and npm i in it.

9) You can create key by command make all and it will be copied to all designed folders

10) Fund the key to do deploymnents


## Test LiquidNFT-Factory Flows

### Main Contract Flow Test:

Run the command in LiquidNFT-Factory-Tests-Scripts/mainContractFlowScript folder:

```

npm run testMainContractFlow <id> <key> <value> <liquidNFTFactoryPackageHash> <cep47PackageHash> <floorAsked> <totalFloor> <paymentTime> <paymentRate> <erc20PackageHash> <keyPath> <mintApproveAmount> <paymentAmountContributeToLocker> <prePaymentAmount> <paymentAmountForPaybackToLocker>

```
Example command:

```

npm run testMainContractFlow 21 name AwesomeNFT f7e686e9086b54918896bda93b490d878abf9a4c35006f68f8fb6ce8811cdff0 5c89f407dacab04f69b704a81c6786b9e115ea3dcea6499d6a95203bece6c406 4000000000 10000000000 86400000 10 4efb977f33caaddc15ebd244a1245b3e930cc9fc898b98792688ba7ecfad4c04 keys 1000000000000 5000000000 1000000000 1000000000

```
### TooLate Payment Liquidate Flow Test:

Run the command in LiquidNFT-Factory-Tests-Scripts/tooLateLiquidateFlowScript folder: 

```
npm run testTooLatePayBackFlow  <id> <key> <value> <liquidNFTFactoryPackageHash> <cep47PackageHash> <floorAsked> <totalFloor> <paymentTime> <paymentRate> <erc20PackageHash> <keyPath> <mintApproveAmount> <paymentAmountContributeToLocker> <prePaymentAmount> <paymentAmountForPaybackToLocker> 

```

Example command:

```
npm run testTooLatePayBackFlow 22 name AwesomeNFT d10359a72bdf42adbe6067b4e7c1c16ccf199e6329a9aeef32c02d40b36ea0fe 5c89f407dacab04f69b704a81c6786b9e115ea3dcea6499d6a95203bece6c406 4000000000 10000000000 120000 10 4efb977f33caaddc15ebd244a1245b3e930cc9fc898b98792688ba7ecfad4c04 keys 1000000000000 5000000000 1000000000 1000000000

```

### Important Notes: 

1) Make sure id is unique everytime you want to test the whole flow again, if flow is not completed by any means don't change the id.

2) If the terminal gets stuck, comment the last successfull deployed functions in the code and run the comand again with the same parameters.


### See Detailed flows:

Readme for mainContract flow:

https://github.com/Scytalelabs-official/CasperLabs-LiquidNFT/blob/main/JsClients/LiquidNFT-Factory-Tests-Scripts/mainContractFlowScript/readme.md

Readme for tooLate liquidate flow:

https://github.com/Scytalelabs-official/CasperLabs-LiquidNFT/blob/main/JsClients/LiquidNFT-Factory-Tests-Scripts/tooLateLiquidateFlowScript/readme.md


## Test JsClients Individually

See all JsClients readme

Readme for cep47 JSClient:

https://github.com/Scytalelabs-official/CasperLabs-LiquidNFT/blob/main/JsClients/casper-cep47/README.md

Readme for erc20 JSClient:

https://github.com/Scytalelabs-official/CasperLabs-LiquidNFT/blob/main/JsClients/uniswapV2Core-erc20/readme.md

Readme for LiquidNFT JSClient: 

https://github.com/Scytalelabs-official/CasperLabs-LiquidNFT/blob/main/JsClients/LiquidNFT/readme.md

Readme for LiquidNFT-Factory JSClient:

https://github.com/Scytalelabs-official/CasperLabs-LiquidNFT/blob/main/JsClients/LiquidNFT-Factory/readme.md
