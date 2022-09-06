import {LiquidNFTFactory} from "./lnftFactory";
let liquidNFTfactory = new LiquidNFTFactory();
liquidNFTfactory.paybackToLocker(process.argv[2],process.argv[3]);

/*
    "Script paybackToLocker comments": {
        "Description" : "use it to payback funds to Locker",
        "Syntax" : "npm run paybackToLocker <lockerPackageHash> <paymentAmount>",
        "Example" : "npm run paybackToLocker 75851cf4cc291babd259927028b7214c47968c1330ea2a84a09f71fbb2b2f4ff 5000000000"
    },
*/

/*Flow to call this function 
Firstly, if you have not done that yet,lock NFT in the locker using funtions createliquidLockerJsClient 
OR createEmptyLocker 
Secondly, if you have not done that yet,contribute to the locker using contributeToLocker function 
more than the floor asked of the locker
Thirdly, enable the locker by calling enableLocker function using LiquidNFT JsClient
*/

/*
Successfull DeployHash For createLiquidLockerJsClient:  56b828d8d55d7ec74acb11946805ea304b68c543dd6b901c24a7fbe45d23891a
Successfull DeployHash For contributeToLocker: e5e92f1c0aee128cf9501d3be73cd62de92b9af3a9f6349e37fd0e8a6439151b
Successfull DeployHash For enableLocker: 9b96edfb708d3a9a2167e7c283e14b28908e7fed068ff1ad4eef1ecf955670cd
Successfull DeployHash For paybackToLocker: 904f545397859339718ddbd42619f7feaec9907818fe62be8e6e44fa6d348314
*/
