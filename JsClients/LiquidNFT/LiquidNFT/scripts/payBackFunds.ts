import { LiquidNFT } from "./lnft";
let liquidNFT = new LiquidNFT();
liquidNFT.payBackFunds(
    process.argv[2],
    process.argv[3],
);

/*
    "Script payBackFunds comments": {
        "Description" : "use it to pay back funds to the locker",
        "Syntax" : "npm run payBackFunds <paymentAmount>",
        "Example" : "npm run payBackFunds 100"
    },
*/