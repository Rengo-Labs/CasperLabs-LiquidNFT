import {LiquidNFTFactory} from "./lnftFactory";
let LiquidNFTfactory = new LiquidNFTFactory();
LiquidNFTfactory.createEmptyLocker(process.argv[2]);

/*
    "Script createEmptyLocker comments": {
        "Description" : "use it to create empty liquidLocker",
        "Syntax" : "npm run createEmptyLocker <erc20PackageHash>",
        "Example" : "npm run createEmptyLocker 56b77636b3af55977cfeea4eb22d18394ee2aa2ba4b2afe3a13a91adc26a1222"
    },
*/

/*Flow to call this function 
No function is required to call before calling this function
*/