import {LiquidNFTFactory} from "./lnftFactory";
let liquidNFTfactory = new LiquidNFTFactory();
liquidNFTfactory.paybackToLocker(process.argv[2],process.argv[3]);

/*
    "Script paybackToLocker comments": {
        "Description" : "use it to payback funds to Locker",
        "Syntax" : "npm run paybackToLocker <lockerPackageHash> <paymentAmount>",
        "Example" : "npm run paybackToLocker 841f21fefb97f759a555255b0a414fda519c6deba4007c3a17ada7dc233552d8 100"
    },
*/