# CasperLabs-LiquidNFT-JsClients

This folder have cep47, erc20 Jsclients and scripts for LiquidNFT-Factory and LiquidNFT JsClients

## Test LiquidNFT-Factory Flows

### Steps:

1) Go to casper-cep47 folder and npm i in it.

2) Go to LiquidNFT folder and npm i in it.

3) Go to LiquidNFT-Factory folder and npm i in it.

4) Go to uniswapV2Core-erc20 folder and npm i in it.

5) Go to LiquidNFT-Factory-Tests-Scripts folder and npm i in it.

6) Generate and paste a keys folder in LiquidNFT-Factory-Tests-Scripts/liquidNFT-Factory-Tests-Scripts folder.

7) Run the command in LiquidNFT-Factory-Tests-Scripts folder: 

npm run testMainContractFlow <id> <key> <value> <liquidNFTFactoryPackageHash> <cep47PackageHash> <floorAsked> <totalFloor> <paymentTime> <paymentRate> <erc20PackageHash> <keyPath> <mintApproveAmount> <paymentAmountContributeToLocker> <prePaymentAmount> <paymentAmountForPaybackToLocker>

Example command:

npm run testMainContractFlow 16 name AwesomeNFT 7fcc17f692368169ba30ea0e90496c4a95d36d7bc8956e2305a86a38bce44675 737588742efd608e68a1ae1bde3955d61e1d3f72b0e85f7755efe2f14363b943 4000000000 10000000000 86400000 10 56b77636b3af55977cfeea4eb22d18394ee2aa2ba4b2afe3a13a91adc26a1222 liquidNFT-Factory-Tests-Scripts/keys 1000000000000 5000000000 1000000000 1000000000

Important Notes: 

1) Make sure id is unique everytime you want to test the whole flow again, if flow is not completed by any means don't change the id.

2) If the terminal gets stuck, comment the last successfull deployed functions in the code in testMainContractFlow file and run the comand again with the same parameters.


## Test JsClients Individually

See all JsClients readme

Readme for cep47 JSClient:

https://github.com/Scytalelabs-official/CasperLabs-LiquidNFT/blob/main/JsClients/casper-cerp47/readme.md

Readme for erc20 JSClient:

https://github.com/Scytalelabs-official/CasperLabs-LiquidNFT/blob/main/JsClients/uniswapV2Core-erc20/readme.md

Readme for LiquidNFT JSClient: 

https://github.com/Scytalelabs-official/CasperLabs-LiquidNFT/blob/main/JsClients/LiquidNFT/readme.md

Readme for LiquidNFT-Factory JSClient:

https://github.com/Scytalelabs-official/CasperLabs-LiquidNFT/blob/main/JsClients/LiquidNFT-Factory/readme.md