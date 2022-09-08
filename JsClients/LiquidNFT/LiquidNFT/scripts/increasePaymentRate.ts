import {LiquidNFT} from "./lnft";
let liquidNFT = new LiquidNFT();
liquidNFT.increasePaymentRate(
    process.argv[2]
);

/*
    "Script increasePaymentRate comments": {
        "Description" : "use it to increase Payment Rate to the locker",
        "Syntax" : "npm run increasePaymentRate <newPaymentRate>",
        "Example" : "npm run increasePaymentRate 100"
    },
*/