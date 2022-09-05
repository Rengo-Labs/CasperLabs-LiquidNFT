import {LiquidNFT} from "./lnft";
let liquidNFT = new LiquidNFT();
liquidNFT.calculatePaybacks(
    process.argv[2],
    process.argv[3]!,
    process.argv[4]!
);

/*
    "Script calculatePaybacks comments": {
        "Description" : "use it to calculate paybacks",
        "Syntax" : "npm run calculatePaybacks <totalValue> <paymentRate> <paymentTime>",
        "Example" : "npm run calculatePaybacks "
    },
*/