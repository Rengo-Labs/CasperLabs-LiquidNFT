import {LiquidNFT} from "./installed";
let liquidNFT = new LiquidNFT();
let tokenIds = [process.argv[2]];
liquidNFT.createLiquidLockerJsClient(
    tokenIds,
    process.argv[3],
    process.argv[4],
    process.argv[5],
    process.argv[6],
    process.argv[7],
    process.argv[8]);

/*
    "Script createLiquidLocker comments": {
        "Description" : "use it to create liquidLocker",
        "Syntax" : "npm run createLiquidLocker <tokenId> <cep47PackageHash> <floorAsked> <totalAsked> <paymentTime> <paymentRate> <erc20PackageHash>",
        "Example" : "npm run createLiquidLocker 1 841f21fefb97f759a555255b0a414fda519c6deba4007c3a17ada7dc233552d8 100 100 10 10 841f21fefb97f759a555255b0a414fda519c6deba4007c3a17ada7dc233552d8"
    },
*/