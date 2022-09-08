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
        "Example" : "npm run makeContribution 100 24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1"
    },
*/