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
        "Example" : "npm run createLiquidLocker 1 737588742efd608e68a1ae1bde3955d61e1d3f72b0e85f7755efe2f14363b943 1000000000 5000000000 86400000 100 56b77636b3af55977cfeea4eb22d18394ee2aa2ba4b2afe3a13a91adc26a1222"
    },
*/

//Mint 