import {LiquidNFT} from "./lnft";
let liquidNFT = new LiquidNFT();
liquidNFT.refundDueSingle(
    process.argv[2]
);

/*
    "Script refundDueSingle comments": {
        "Description" : "use it to refundDueSingle",
        "Syntax" : "npm run rrefundDueSingle <refundAddressAccountHash>",
        "Example" : "npm run refundDueSingle 24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1"
    },
*/