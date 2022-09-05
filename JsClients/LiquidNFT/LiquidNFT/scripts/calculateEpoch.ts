import {LiquidNFT} from "./lnft";
let liquidNFT = new LiquidNFT();
liquidNFT.calculateEpoch(
    process.argv[2],
    process.argv[3]!,
    process.argv[4]!
);

/*
    "Script calculateEpoch comments": {
        "Description" : "use it to calculate epoch",
        "Syntax" : "npm run calculateEpoch <totalValue> <paymentRate> <paymentTime>",
        "Example" : "npm run calculateEpoch "
    },
*/