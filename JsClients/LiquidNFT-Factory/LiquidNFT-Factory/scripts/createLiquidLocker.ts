import {LiquidNFTFactory} from "./lnftFactory";
let liquidNFTfactory = new LiquidNFTFactory();
let tokenIds = [process.argv[2]];
liquidNFTfactory.createLiquidLockerJsClient(
    tokenIds,
    process.argv[3]!,
    process.argv[4]!,
    process.argv[5]!,
    process.argv[6]!,
    process.argv[7]!,
    process.argv[8]!);

/*
    "Script createLiquidLocker comments": {
        "Description" : "use it to create liquidLocker",
        "Syntax" : "npm run createLiquidLocker <tokenId> <cep47PackageHash> <floorAsked> <totalAsked> <paymentTime> <paymentRate> <erc20PackageHash>",
        "Example" : "npm run createLiquidLocker 13 737588742efd608e68a1ae1bde3955d61e1d3f72b0e85f7755efe2f14363b943 1000000000 5000000000 86400000 3600 56b77636b3af55977cfeea4eb22d18394ee2aa2ba4b2afe3a13a91adc26a1222"
    },
*/

/*Flow to call this function 
Firstly, Mint NFT against Onwer in cep47 JsClient using mintOneToken function
Secondly, Approve NFT against LiquidNFT Factory Package Hash in cep47 JsClient using approveOneToken function
*/

/*
Successfull DeployHash For mintOneToken: 4af852da890104582a535d90ea369a5b9a1bb1fcc19874fdaf9f8f6dc4abedae
Successfull DeployHash For approveOneToken: c09be8694e1295965aa63e26679fb4fdaff8b79252abc64f6ce7e4cf9bed1920
Successfull DeployHash For createLiquidLockerJsClient: 56b828d8d55d7ec74acb11946805ea304b68c543dd6b901c24a7fbe45d23891a
*/