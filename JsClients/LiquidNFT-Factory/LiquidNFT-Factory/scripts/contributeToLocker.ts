import {LiquidNFTFactory} from "./lnftFactory";
let LiquidNFTfactory = new LiquidNFTFactory();
LiquidNFTfactory.contributeToLocker(process.argv[2]!,process.argv[3]!);

/*
    "Script contributeToLocker comments": {
        "Description" : "use it to contribute to Locker",
        "Syntax" : "npm run contributeToLocker <lockerPackageHash> <paymentAmount>",
        "Example" : "npm run contributeToLocker d774f03ded1fb722e729fdebbb117f4ae39d52c5817d79f4155ff6254d6d294a 5000000000"
    },
*/

/*Flow to call this function 
Firstly, if you have not done that yet,lock NFT in the locker using funtions createliquidLockerJsClient
OR createEmptyLocker 
IMPORTANT NOTE: paymentTime and paymentRate values should be in proportion to each other
(Neither too big nor too low, else you will get user errors in other functions)

Secondly, Mint erc20 tokens against Onwer in erc20 JsClient using mint function (which you want to contribute)

Thirdly, Approve erc20 tokens against LiquidNFT Factory Package Hash in erc20 JsClient using approve
function(which you want to contribute)
*/

/*
Successfull DeployHash For createLiquidLockerJsClient: 437c526d50524ca412b36578033948d141d30af3be9ef3ffe1bbead477228692
Successfull DeployHash For mint: 7f59b10de96cf5a2b8b298666cf301766fed3b9aeb965baf2ebbf9bccfd1f8d7
Successfull DeployHash For approve: c2d60eef9ec20733619ed492132758abc7c6609722cab698a36c65440dcbabb8
Successfull DeployHash For contributeToLocker: c1dcf343f2c5c392e898a4a49ec4d4c93b68ae1310070b71a45de6ca36e69b37
*/