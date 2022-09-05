import {LiquidNFT} from "./lnft";
let liquidNFT = new LiquidNFT();
liquidNFT.makeContribution(
    process.argv[2],
    process.argv[3]!
);

/*
    "Script makeContribution comments": {
        "Description" : "use it to make contribution to locker",
        "Syntax" : "npm run makeContribution <tokenAmount> <tokenHolder>",
        "Example" : "npm run makeContribution "
    },
*/