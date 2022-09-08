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

/* 
Successfull DeployHash For createEmptyLocker: eb94c61bf0e06775713692c6326c0ee964fdbfa573c9d94c31f78daef206521b
Successfull DeployHash For createLiquidLockerJsClient: d73143a193da7a2ffa3bcc7b50918431c16d5090b75c349821c2d89b553b5065
*/