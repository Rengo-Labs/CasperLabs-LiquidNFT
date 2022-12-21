import { LiquidNFT } from "./lnft";
let liquidNFT = new LiquidNFT();
liquidNFT.refundDueExpired(
    process.argv[2]
);

/*
    "Script refundDueExpired comments": {
        "Description" : "use it to disable due refund",
        "Syntax" : "npm run refundDueExpired <refundAddressAccountHash>",
        "Example" : "npm run refundDueExpired 24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1"
    },
*/