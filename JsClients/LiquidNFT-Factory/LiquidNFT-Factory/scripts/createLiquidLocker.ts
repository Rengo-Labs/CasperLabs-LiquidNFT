import {LiquidNFTFactory} from "./lnftFactory";
let liquidNFTfactory = new LiquidNFTFactory();
let tokenIds = [process.argv[2]];
liquidNFTfactory.createLiquidLockerJsClient(
    tokenIds,
    process.argv[3]!,
    process.argv[4]!,
    process.argv[5]!,
    process.argv[6]!,
    process.argv[7]!,
    process.argv[8]!);

/*
    "Script createLiquidLocker comments": {
        "Description" : "use it to create liquidLocker",
        "Syntax" : "npm run createLiquidLocker <tokenId> <cep47PackageHash> <floorAsked> <totalAsked> <paymentTime> <paymentRate> <erc20PackageHash>",
        "Example" : "npm run createLiquidLocker 15 737588742efd608e68a1ae1bde3955d61e1d3f72b0e85f7755efe2f14363b943 4000000000 10000000000 86400000 10 56b77636b3af55977cfeea4eb22d18394ee2aa2ba4b2afe3a13a91adc26a1222"
    },
*/

/*Flow to call this function 
Firstly, Mint NFT against Onwer in cep47 JsClient using mintOneToken function

Secondly, Approve NFT against LiquidNFT Factory Package Hash in cep47 JsClient using approveOneToken function

IMPORTANT NOTE: paymentTime and paymentRate values should be in proportion to each other
(Neither too big nor too low, else you will get user errors in other functions)
*/

/*
Successfull DeployHash For mintOneToken: a95b2e1ebff578dad966f69e0524462f14da8f0fdfdbf45b40aebbce825ab18e
Successfull DeployHash For approveOneToken: 3ace7cb5d2211ea3d82be8a71ceb0453f60eeec959b51a30fa47caba61d49d80
Successfull DeployHash For createLiquidLockerJsClient: 437c526d50524ca412b36578033948d141d30af3be9ef3ffe1bbead477228692
*/