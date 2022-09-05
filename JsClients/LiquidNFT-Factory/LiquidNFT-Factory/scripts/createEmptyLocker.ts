import {LiquidNFTFactory} from "./lnftFactory";
let LiquidNFTfactory = new LiquidNFTFactory();
LiquidNFTfactory.createEmptyLocker(process.argv[2]);

/*
    "Script createEmptyLocker comments": {
        "Description" : "use it to create liquidLocker",
        "Syntax" : "npm run createEmptyLocker <erc20PackageHash>",
        "Example" : "npm run createEmptyLocker 841f21fefb97f759a555255b0a414fda519c6deba4007c3a17ada7dc233552d8"
    },
*/