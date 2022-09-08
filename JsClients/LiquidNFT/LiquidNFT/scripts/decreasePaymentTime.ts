import {LiquidNFT} from "./lnft";
let liquidNFT = new LiquidNFT();
liquidNFT.decreasePaymentTime(
    process.argv[2]
);

/*
    "Script decreasePaymentTime comments": {
        "Description" : "use it to decrease Payment Rate to the locker",
        "Syntax" : "npm run decreasePaymentTime <newPaymentRate>",
        "Example" : "npm run decreasePaymentTime 100"
    },
*/