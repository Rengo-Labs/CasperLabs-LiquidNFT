import {LiquidNFTFactory} from "../../../LiquidNFT-Factory/LiquidNFT-Factory/scripts/lnftFactory";
import {LiquidNFT} from "../../../LiquidNFT/LiquidNFT/scripts/lnft";
import {Cep47} from '../../../casper-cep47/scripts/cep47';
import {ERC20} from '../../../uniswapV2Core-erc20/ERC20/scripts/erc20';
import * as fs from 'fs';

async function testMainContractFlow(
    id: string[],
    key: string,
    value: string,
    liquidNFTFactoryPackageHash: string,
    cep47PackageHash: string,
    floorAsked: string,
    totalFloor: string,
    paymentTime: string,
    paymentRate: string,
    erc20PackageHash: string,
    keyPath: string,
    mintApproveAmount: string,
    paymentAmountContributeToLocker: string,
    prePaymentAmount: string,
    paymentAmountForPaybackToLocker: string)
{
    //initializing all classes
    let liquidNFTfactory = new LiquidNFTFactory();
    let liquidNFT = new LiquidNFT();
    let cep47 = new Cep47();
    let erc20 = new ERC20();

    // Call mintOneToken
    let map = new Map<string,string>().set(key,value);
    let arrayOfMaps = [map];
    let tokenIds = id;
    
    await cep47.mint(tokenIds,arrayOfMaps);

    //Call approveOneToken
    await cep47.approve(liquidNFTFactoryPackageHash,tokenIds);

    // Call createliquidLockerJsClient
    await liquidNFTfactory.createLiquidLockerJsClient(
        tokenIds,
        cep47PackageHash,
        floorAsked, 
        totalFloor,
        paymentTime, 
        paymentRate, 
        erc20PackageHash);

    // Call lockerHashes
    await liquidNFTfactory.lockerHashes();
    let _lockerPackageHash = fs.readFileSync('liquidNFTPackageHash','utf8');
    let lockerPackageHash=_lockerPackageHash.split("-").pop()!; 
    
    // Call erc20 mint
    await erc20.mint(keyPath,mintApproveAmount);

    // Call erc20 approve
    await erc20.approve(keyPath,mintApproveAmount);

    // Call contributeToLocker
    await liquidNFTfactory.contributeToLocker(lockerPackageHash,paymentAmountContributeToLocker);    

    // Call enableLocker
    await liquidNFT.enableLocker(prePaymentAmount);

    // Call paybackToLocker
    await liquidNFTfactory.paybackToLocker(lockerPackageHash,paymentAmountForPaybackToLocker);

}
testMainContractFlow(
    [process.argv[2]!],
    process.argv[3]!,
    process.argv[4]!,
    process.argv[5]!,
    process.argv[6]!,
    process.argv[7]!,
    process.argv[8]!,
    process.argv[9]!,
    process.argv[10]!,
    process.argv[11]!,
    process.argv[12]!,
    process.argv[13]!,
    process.argv[14]!,
    process.argv[15]!,
    process.argv[16]!);


/*
    "Script testMainContractFlow comments": {
    "Description" : "use it to test main contract flow",
    "Syntax" : "npm run testMainContractFlow  <id> <key> <value> <liquidNFTFactoryPackageHash> <cep47PackageHash> <floorAsked> <totalFloor> <paymentTime> <paymentRate> <erc20PackageHash> <keyPath> <mintApproveAmount> <paymentAmountContributeToLocker> <prePaymentAmount> <paymentAmountForPaybackToLocker> ",
    "Example" : "npm run testMainContractFlow 17 name AwesomeNFT 7fcc17f692368169ba30ea0e90496c4a95d36d7bc8956e2305a86a38bce44675 737588742efd608e68a1ae1bde3955d61e1d3f72b0e85f7755efe2f14363b943 4000000000 10000000000 86400000 10 56b77636b3af55977cfeea4eb22d18394ee2aa2ba4b2afe3a13a91adc26a1222 /keys 1000000000000 5000000000 1000000000 1000000000"
  },
*/


/*Main Flow of the factory contract 

1) Mint NFT against Onwer in cep47 JsClient using mintOneToken function

2) Approve NFT against LiquidNFT Factory Package Hash in cep47 JsClient using approveOneToken function

3) Lock NFT in the locker using funtion createliquidLockerJsClient

IMPORTANT NOTE: paymentTime and paymentRate values should be in proportion to each other
(Neither too big nor too low, else you will get user errors in other functions)

4) Mint erc20 tokens against Onwer in erc20 JsClient using mint function (which you want to contribute)

5) Approve erc20 tokens against LiquidNFT Factory Package Hash in erc20 JsClient using approve
function(which you want to contribute)

6) Contribute to the locker using contributeToLocker function

7) Enable the locker by calling enableLocker function using LiquidNFT JsClient
IMPORTANT NOTE: Make sure floor asked has reached

8) Call payBackToLocker function 

*/

/*
Successfull DeployHash For mintOneToken: a95b2e1ebff578dad966f69e0524462f14da8f0fdfdbf45b40aebbce825ab18e
Successfull DeployHash For approveOneToken: 3ace7cb5d2211ea3d82be8a71ceb0453f60eeec959b51a30fa47caba61d49d80
Successfull DeployHash For createLiquidLockerJsClient: 437c526d50524ca412b36578033948d141d30af3be9ef3ffe1bbead477228692
Successfull DeployHash For mint: 7f59b10de96cf5a2b8b298666cf301766fed3b9aeb965baf2ebbf9bccfd1f8d7
Successfull DeployHash For approve: c2d60eef9ec20733619ed492132758abc7c6609722cab698a36c65440dcbabb8
Successfull DeployHash For contributeToLocker: c1dcf343f2c5c392e898a4a49ec4d4c93b68ae1310070b71a45de6ca36e69b37
Successfull DeployHash For enableLocker: b34ab4cc4cf3745f55ce05e0da491020c317bb1468ecbb49abe40aa5b8ac9956
Successfull DeployHash For paybackToLocker: e610daa74f2591b34c30f44a8ee6e41d15cff06d2762757cbbbcc81939401162
*/