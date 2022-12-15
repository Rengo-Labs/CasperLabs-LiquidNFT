import {LiquidNFT} from "./lnft";
let liquidNFT = new LiquidNFT();
liquidNFT.donateFunds(
    process.argv[2]
);

/*
    "Script donateFunds comments": {
        "Description" : "use it to donate funds to the locker",
        "Syntax" : "npm run donateFunds <donationAmount>",
        "Example" : "npm run donateFunds 100"
    },
*/