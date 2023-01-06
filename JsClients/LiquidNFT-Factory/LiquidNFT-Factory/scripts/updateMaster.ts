import {LiquidNFTFactory} from "./lnftFactory";
let liquidNFTfactory = new LiquidNFTFactory();
liquidNFTfactory.updateMaster(process.argv[2]!);

/*
    "Script updateMaster comments": {
        "Description" : "use it to updateMaster",
        "Syntax" : "npm run updateMaster <newMasterAccountHash> ",
        "Example" : "npm run updateMaster "
    },
*/

/*Flow to call this function 
No function is required to call before calling this function
*/