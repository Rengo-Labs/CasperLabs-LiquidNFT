import {LiquidNFTFactory} from "./lnftFactory";
let liquidNFTfactory = new LiquidNFTFactory();
liquidNFTfactory.donateToLocker(process.argv[2],process.argv[3]);

/*
    "Script donateToLocker comments": {
        "Description" : "use it to donate to Locker",
        "Syntax" : "npm run donateToLocker <lockerPackageHash> <donationAmount>",
        "Example" : "npm run donateToLocker ab3fa4bc9717719a27e7bad8eb95391a8a402d70f0dfab811445e0003dc1ef1e 10"
    },
*/

/*Flow to call this function 
Firstly, if you have not done that yet,lock NFT in the locker using funtions createliquidLockerJsClient
OR createEmptyLocker 
Secondly, Mint erc20 tokens against Onwer in erc20 JsClient using mint function (which you want to donate)
Thirdly, Approve erc20 tokens against LiquidNFT Factory Package Hash in erc20 JsClient using approve
function(which you want to donate)
*/

/*
Successfull DeployHash For createLiquidLockerJsClient: d73143a193da7a2ffa3bcc7b50918431c16d5090b75c349821c2d89b553b5065
Successfull DeployHash For mint: a429b7ca9f882750162c375e90c06c7b2fc13e259dbc41145718f06eadec6851
Successfull DeployHash For approve: 7ee8c7c197c49f64f92dda420ec55e863fa95f4e15d88e139756b976d691e19e
Successfull DeployHash For donateToLocker: 7768af95dc4e4648344dbdecd2eeb7d096976ef4d8c406f090204a494827d4a5
*/