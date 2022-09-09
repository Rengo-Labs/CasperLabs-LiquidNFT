import {LiquidNFTFactory} from "../../../LiquidNFT-Factory/LiquidNFT-Factory/scripts/lnftFactory";
import {LiquidNFT} from "../../../LiquidNFT/LiquidNFT/scripts/lnft";
import {Cep47} from '../../../casper-cep47/scripts/cep47';
import {ERC20} from '../../../uniswapV2Core-erc20/ERC20/scripts/erc20';
import * as fs from 'fs';
import { sleep } from "../../../casper-cep47/src/utils";

async function testTooLatePayBackFlow(
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
    let result = await liquidNFTfactory.lockerHashes();
    fs.writeFileSync('liquidNFTContractHash',result.contractHash,{encoding:'utf8',flag:'w'});
    fs.writeFileSync('liquidNFTPackageHash',result.packageHash,{encoding:'utf8',flag:'w'});

    let _lockerPackageHash = fs.readFileSync('liquidNFTPackageHash','utf8');
    let lockerPackageHash=_lockerPackageHash.split("-").pop()!; 
    
    // Call erc20 mint
    await erc20.mint(keyPath,mintApproveAmount);

    // Call erc20 approve
    await erc20.approve(liquidNFTFactoryPackageHash,mintApproveAmount);
    
    //You Have to call it within 15 mintues after liquidLockerCreation
    // Call contributeToLocker
    await liquidNFTfactory.contributeToLocker(lockerPackageHash,paymentAmountContributeToLocker);    

    let liquidNFT = new LiquidNFT();
    // Call enableLocker
    await liquidNFT.enableLocker(prePaymentAmount);
    
    //wait to forcefully make payment late
    await sleep(1200000);
    
    //You have to wait for 20 mintues to make payment late
    // Call paybackToLocker
    await liquidNFTfactory.paybackToLocker(lockerPackageHash,paymentAmountForPaybackToLocker);

    // Call paybackToLocker
    await liquidNFT.liquidateLocker();

}
testTooLatePayBackFlow(
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
    "Script testTooLatePayBackFlow comments": {
    "Description" : "use it to test too late pay back flow",
    "Syntax" : "npm run testTooLatePayBackFlow  <id> <key> <value> <liquidNFTFactoryPackageHash> <cep47PackageHash> <floorAsked> <totalFloor> <paymentTime> <paymentRate> <erc20PackageHash> <keyPath> <mintApproveAmount> <paymentAmountContributeToLocker> <prePaymentAmount> <paymentAmountForPaybackToLocker> ",
    "Example" : "npm run testTooLatePayBackFlow 20 name AwesomeNFT d10359a72bdf42adbe6067b4e7c1c16ccf199e6329a9aeef32c02d40b36ea0fe 5c89f407dacab04f69b704a81c6786b9e115ea3dcea6499d6a95203bece6c406 4000000000 10000000000 120000 10 4efb977f33caaddc15ebd244a1245b3e930cc9fc898b98792688ba7ecfad4c04 keys 1000000000000 5000000000 1000000000 1000000000"
  },
*/


/* Flow of should be able to liquadateLocker when payment is late:

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

8) wait 20 minutes

9) Call payBackToLocker function, you will receive user error: 141 (TooLate)

10) Call liquidateLocker function 

*/

/*
Successfull DeployHash For mintOneToken: 400d49165137ac023c5017a885ad007006d173362394e65cc37daa3b600ba635
Successfull DeployHash For approveOneToken: 05a9e3061b47538f5bf4b3bd7eab6cb73839995ed86e38fed0f5d0b9fa6b3d8d
Successfull DeployHash For createLiquidLockerJsClient: c4fb65891dbf4f399baaa058327340740f9baf93e87cbf72bfd0089c9060008a
Successfull DeployHash For mint: 51652aaaae3fc6ca42008a1af81afabef9d204340fab4fba4c6695ccd527bd1b
Successfull DeployHash For approve: ed24d4231092b6dae7d4ca98e106c1483133a7017e1ecb7a6690bf52b7bab115
Successfull DeployHash For contributeToLocker: 0532d8b5ffe8d11b9662fc3cd5bcda98d5180a417e9eeb17d968720121220e4c
Successfull DeployHash For enableLocker: 297a06b50a5cbfd03cf13593e9bb433e1da35344f1be40a900a3a12842893a97
Failed DeployHash For paybackToLocker: f2bdcb3fd6355d61c70df7bbaae85bfa3b074653478aa389e30b0464c1aa8b9f
Successfull DeployHash For liquidateLocker: 1d600938f5d3f8e2c047f6dfea7ce14b08176cc7c4cc5d87c18e9ac35e9bf742
*/