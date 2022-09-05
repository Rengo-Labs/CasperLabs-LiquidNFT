import {LiquidNFTFactory} from "./lnftFactory";
let LiquidNFTfactory = new LiquidNFTFactory();
LiquidNFTfactory.contributeToLocker(process.argv[2]!,process.argv[3]!);

/*
    "Script contributeToLocker comments": {
        "Description" : "use it to contribute to Locker",
        "Syntax" : "npm run contributeToLocker <lockerPackageHash> <paymentAmount>",
        "Example" : "npm run contributeToLocker 841f21fefb97f759a555255b0a414fda519c6deba4007c3a17ada7dc233552d8 100"
    },
*/