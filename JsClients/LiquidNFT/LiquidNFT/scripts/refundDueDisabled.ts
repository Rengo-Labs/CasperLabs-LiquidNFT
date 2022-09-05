import {LiquidNFT} from "./lnft";
let liquidNFT = new LiquidNFT();
liquidNFT.refundDueDisabled(
    process.argv[2]
);

/*
    "Script refundDueDisabled comments": {
        "Description" : "use it to disable due refund",
        "Syntax" : "npm run refundDueDisabled <refundAddressAccountHash>",
        "Example" : "npm run refundDueDisabled 24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1"
    },
*/