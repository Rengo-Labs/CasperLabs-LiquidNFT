import {LiquidNFTFactory} from "./lnftFactory";
let liquidNFTfactory = new LiquidNFTFactory();
liquidNFTfactory.paybackToLocker(process.argv[2],process.argv[3]);

/*
    "Script paybackToLocker comments": {
        "Description" : "use it to payback funds to Locker",
        "Syntax" : "npm run paybackToLocker <lockerPackageHash> <paymentAmount>",
        "Example" : "npm run paybackToLocker 5b8e4c95146e85afe8861832898b460133e153a102cc5b5cace28f6aa60dff08 1000000000"
    },
*/

/*Flow to call this function 
Firstly, if you have not done that yet,lock NFT in the locker using funtions createliquidLockerJsClient 
IMPORTANT NOTE: paymentTime and paymentRate values should be in proportion to each other
(Neither too big nor too low, else you will get user errors)

Secondly, if you have not done that yet,contribute to the locker using contributeToLocker function 
more than the floor asked of the locker

Thirdly, make sure floor asked has reached

Fourthly, enable the locker by calling enableLocker function using LiquidNFT JsClient
*/

/*
Successfull DeployHash For createLiquidLockerJsClient: 437c526d50524ca412b36578033948d141d30af3be9ef3ffe1bbead477228692
Successfull DeployHash For contributeToLocker: c1dcf343f2c5c392e898a4a49ec4d4c93b68ae1310070b71a45de6ca36e69b37
Successfull DeployHash For enableLocker: b34ab4cc4cf3745f55ce05e0da491020c317bb1468ecbb49abe40aa5b8ac9956
Successfull DeployHash For paybackToLocker: e610daa74f2591b34c30f44a8ee6e41d15cff06d2762757cbbbcc81939401162
*/
