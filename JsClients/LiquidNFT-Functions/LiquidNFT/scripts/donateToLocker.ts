import {LiquidNFT} from "./installed";
let liquidNFT = new LiquidNFT();
liquidNFT.donateToLocker(process.argv[1],process.argv[2]);

/*
    "Script donateToLocker comments": {
        "Description" : "use it to donate to Locker",
        "Syntax" : "npm run donateToLocker <lockerPackageHash> <donationAmount>",
        "Example" : "npm run donateToLocker 841f21fefb97f759a555255b0a414fda519c6deba4007c3a17ada7dc233552d8 100"
    },
*/