import {LiquidNFT} from "./lnft";
let liquidNFT = new LiquidNFT();
liquidNFT.enableLocker(
    process.argv[2]
);

/*
    "Script enableLocker comments": {
        "Description" : "use it to enable the locker",
        "Syntax" : "npm run enableLocker <prePaymentAmount>",
        "Example" : "npm run enableLocker 1200000"
    },
*/