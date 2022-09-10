# CasperLabs-LiquidNFT-JsClients

This folder have cep47, erc20 Jsclients and scripts for LiquidNFT-Factory and LiquidNFT JsClients

## Test LiquidNFT-Factory Flows

### Steps:

1) Go to JsClientsForFrontend/LiquidNFT folder and npm i in it.

2) Go to JsClientsForFrontend/LiquidNFT-Factory folder and npm i in it.

3) Go to casper-cep47 folder and npm i in it.

4) Go to LiquidNFT folder and npm i in it.

5) Go to LiquidNFT-Factory folder and npm i in it.

6) Go to uniswapV2Core-erc20 folder and npm i in it.

7) Go to LiquidNFT-Factory-Tests-Scripts folder and npm i in it.

8) Generate and paste a keys folder in LiquidNFT-Factory-Tests-Scripts/liquidNFT-Factory-Tests-Scripts folder.

9) Run the command in LiquidNFT-Factory-Tests-Scripts folder: 

npm run testMainContractFlow <id> <key> <value> <liquidNFTFactoryPackageHash> <cep47PackageHash> <floorAsked> <totalFloor> <paymentTime> <paymentRate> <erc20PackageHash> <keyPath> <mintApproveAmount> <paymentAmountContributeToLocker> <prePaymentAmount> <paymentAmountForPaybackToLocker>

Example command:

npm run testMainContractFlow 19 name AwesomeNFT f7e686e9086b54918896bda93b490d878abf9a4c35006f68f8fb6ce8811cdff0 5c89f407dacab04f69b704a81c6786b9e115ea3dcea6499d6a95203bece6c406 4000000000 10000000000 86400000 10 4efb977f33caaddc15ebd244a1245b3e930cc9fc898b98792688ba7ecfad4c04 liquidNFT-Factory-Tests-Scripts/keys 1000000000000 5000000000 1000000000 1000000000

Important Notes: 

1) Make sure id is unique everytime you want to test the whole flow again, if flow is not completed by any means don't change the id.

2) If the terminal gets stuck, comment the last successfull deployed functions in the code in testMainContractFlow file and run the comand again with the same parameters.


## Test JsClients Individually

See all JsClients readme

Readme for cep47 JSClient:

https://github.com/Rengo-Labs/CasperLabs-LiquidNFT/blob/main/JsClients/casper-cep47/README.md

Readme for erc20 JSClient:

https://github.com/Rengo-Labs/CasperLabs-LiquidNFT/blob/main/JsClients/uniswapV2Core-erc20/readme.md

Readme for LiquidNFT JSClient: 

https://github.com/Rengo-Labs/CasperLabs-LiquidNFT/blob/main/JsClients/LiquidNFT/readme.md

Readme for LiquidNFT-Factory JSClient:

https://github.com/Rengo-Labs/CasperLabs-LiquidNFT/blob/main/JsClients/LiquidNFT-Factory/readme.md
