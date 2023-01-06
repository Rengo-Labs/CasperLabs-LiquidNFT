import {LiquidNFT} from "./lnft";
let liquidNFT = new LiquidNFT();
liquidNFT.penaltyAmount(
    process.argv[2],
    process.argv[3]!
);

/*
    "Script penaltyAmount comments": {
        "Description" : "use it to check penaltyAmount",
        "Syntax" : "npm run penaltyAmount <totalCollected> <lateDaysAmount>",
        "Example" : "npm run penaltyAmount 100 100"
    },
*/