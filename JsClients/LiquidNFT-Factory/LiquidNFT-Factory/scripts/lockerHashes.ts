import {LiquidNFTFactory} from "./lnftFactory";
let liquidNFTfactory = new LiquidNFTFactory();
liquidNFTfactory.lockerHashes();

/*
    "Script lockerHashes comments": {
        "Description" : "use it to get package and contract hash of locker",
        "Syntax" : "npm run lockerHashes",
    },
*/

/*Flow to call this function 
createEmptyLocker OR  createLiquidLocker function is required to call before calling this function
*/