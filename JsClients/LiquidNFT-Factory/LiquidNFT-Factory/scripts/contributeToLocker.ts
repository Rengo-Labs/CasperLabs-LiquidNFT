import {LiquidNFTFactory} from "./lnftFactory";
let LiquidNFTfactory = new LiquidNFTFactory();
LiquidNFTfactory.contributeToLocker(process.argv[2]!,process.argv[3]!);

/*
    "Script contributeToLocker comments": {
        "Description" : "use it to contribute to Locker",
        "Syntax" : "npm run contributeToLocker <lockerPackageHash> <paymentAmount>",
        "Example" : "npm run contributeToLocker 75851cf4cc291babd259927028b7214c47968c1330ea2a84a09f71fbb2b2f4ff 3000000000"
    },
*/

/*Flow to call this function 
Firstly, if you have not done that yet,lock NFT in the locker using funtions createliquidLockerJsClient
OR createEmptyLocker 
Secondly, Mint erc20 tokens against Onwer in erc20 JsClient using mint function (which you want to contribute)
Thirdly, Approve erc20 tokens against LiquidNFT Factory Package Hash in erc20 JsClient using approve
function(which you want to contribute)
*/

/*
Successfull DeployHash For createLiquidLockerJsClient: 56b828d8d55d7ec74acb11946805ea304b68c543dd6b901c24a7fbe45d23891a
Successfull DeployHash For mint: bf0637d34c9dc396d234d19ea97d46e7172fcb386cf29f0d23bdefa25b3a179d
Successfull DeployHash For approve: 562cba33756bee9cb05cb97c39abe21dfd2bf2b80fd46a3d24bb8ddce5b78d17
Successfull DeployHash For contributeToLocker: e5e92f1c0aee128cf9501d3be73cd62de92b9af3a9f6349e37fd0e8a6439151b
*/