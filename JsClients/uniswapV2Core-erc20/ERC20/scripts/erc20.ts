import { config } from "dotenv";
config();
import { ERC20Client ,utils, constants} from "../src";
import { sleep, getDeploy } from "./utils";
import * as jsutils from "../../utils/utils";

import {
  CLValueBuilder,
  Keys,
  CLPublicKey,
  CLAccountHash,
  CLPublicKeyType,
  Contracts,
  CLByteArray
} from "casper-js-sdk";

import * as fs from 'fs';


const {
  ERC20_TOKEN_NAME,
  ERC20_TOKEN_SYMBOL,
  ERC20_DECIMALS,
  ERC20_TOTAL_SUPPLY,
  ERC20_INSTALL_PAYMENT_AMOUNT,
  ERC20_WASM_PATH,
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  ERC20_MASTER_KEY_PAIR_PATH,
  ERC20_CONTRACT_NAME,
  MINT_PAYMENT_AMOUNT,
  MINT_AMOUNT,
  BURN_PAYMENT_AMOUNT,
  BURN_AMOUNT,
  APPROVE_PAYMENT_AMOUNT,
  APPROVE_AMOUNT,
  TRANSFER_PAYMENT_AMOUNT,
  TRANSFER_AMOUNT,
  TRANSFER_FROM_PAYMENT_AMOUNT,
  TRANSFER_FROM_AMOUNT,
  TOKEN0_CONTRACT,
  TOKEN1_CONTRACT,
  TOKEN1_CONTRACT_PACKAGE,
  PAIR_CONTRACT,
  PACKAGE_HASH,
  AMOUNT_A_DESIRED,
  AMOUNT_B_DESIRED,
  MASTER_KEY_PAIR_PATH,
  PAIR_CONTRACT_PACKAGE
} = process.env;


const KEYS = Keys.Ed25519.parseKeyFiles(
  `${ERC20_MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${ERC20_MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

class ERC20{
    contractHash: string;
    erc20Client: ERC20Client; 

    constructor(){
      this.erc20Client = new ERC20Client(
        NODE_ADDRESS!,
        CHAIN_NAME!,
        EVENT_STREAM_ADDRESS!
      );

          let _contractHash = fs.readFileSync('ERC20ContractHash','utf8');
          this.contractHash = _contractHash.split("-").pop()!;
          //console.log(this.contractHash);
    }
    /*keyPath:string, mintAmount: string*/

    mint = async (keyPath:string,mintAmount: string) => {
      
      await this.erc20Client.setContractHash(this.contractHash!);
      let k = jsutils.getKeys(keyPath);
      const mintDeployHash = await this.erc20Client.mint(
        k,
        k.publicKey,
        mintAmount!,
        MINT_PAYMENT_AMOUNT!
      );
      console.log("... Mint deploy hash: ", mintDeployHash);

      await getDeploy(NODE_ADDRESS!, mintDeployHash);
      console.log("... Token minted successfully.");
    }

    approve = async (packageHash:string,approveAmount: string) => {
      
      await this.erc20Client.setContractHash(this.contractHash!);
      const approveDeployHash = await this.erc20Client.approve(
        KEYS,
        packageHash,
        approveAmount!,
        MINT_PAYMENT_AMOUNT!
      );
      console.log("... Approve deploy hash: ", approveDeployHash);

      await getDeploy(NODE_ADDRESS!, approveDeployHash);
      console.log("... Token approved successfully.");
    }

    deploy = async () => {
      
      let contractName = ERC20_CONTRACT_NAME + jsutils.getDeploymentCount();
      jsutils.updateDeploymentCount();
    
      let k = jsutils.getKeys("ERC20/keys");
    
      const installDeployHash = await this.erc20Client.install(
        k,
        ERC20_TOKEN_NAME!,
        ERC20_TOKEN_SYMBOL!,
        ERC20_DECIMALS!,
        ERC20_TOTAL_SUPPLY!,
    
        contractName!,
        ERC20_INSTALL_PAYMENT_AMOUNT!,
        ERC20_WASM_PATH!
      );
    
      console.log(`... Contract installation deployHash: ${installDeployHash}`);
    
      await getDeploy(NODE_ADDRESS!, installDeployHash);
    
      console.log(`... Contract installed successfully.`);
    
      let accountInfo = await utils.getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);
    
      console.log(`... Account Info: `);
      console.log(JSON.stringify(accountInfo, null, 2));
    
      const contractHash = await utils.getAccountNamedKeyValue(
        accountInfo,
        `${contractName!}_contract_hash`
      );
      fs.writeFileSync('ERC20ContractHash',contractHash,{encoding:'utf8',flag:'w'});
      fs.writeFileSync('.././LiquidNFT-Factory-Tests-Scripts/ERC20ContractHash',contractHash,{encoding:'utf8',flag:'w'});
      console.log(`... Contract Hash: ${contractHash}`);
    
      const packageHash = await utils.getAccountNamedKeyValue(
        accountInfo,
        `${contractName!}_package_hash`
      );
    
      fs.writeFileSync('ERC20PackageHash',packageHash,{encoding:'utf8',flag:'w'});
      fs.writeFileSync('.././LiquidNFT-Factory-Tests-Scripts/ERC20PackageHash',packageHash,{encoding:'utf8',flag:'w'});
      console.log(`... Package Hash: ${packageHash}`);
      
    };

    name = async () => {
        await this.erc20Client.setContractHash(this.contractHash!);
        const name = await this.erc20Client.name();
        console.log(`... Contract name: ${name}`);
    }
    
    symbol = async () => {
      await this.erc20Client.setContractHash(this.contractHash!);
      const symbol = await this.erc20Client.symbol();
      console.log(`... Contract symbol: ${symbol}`);
    }
      
    decimal = async () => {
      await this.erc20Client.setContractHash(this.contractHash!);
      const decimal = await this.erc20Client.decimal();
      console.log(`... Contract decimal: ${decimal}`);
    }

    totalSupply = async () => {
      await this.erc20Client.setContractHash(this.contractHash!);
      let totalSupply = await this.erc20Client.totalSupply();
      console.log(`... Total supply: ${totalSupply}`);
    }

    balanceOf = async (address: string) => {
      await this.erc20Client.setContractHash(this.contractHash!);
      let balance = await this.erc20Client.balanceOfcontract(address!);
      console.log(`... Balance: ${balance}`);
    }

    transfer = async (to: string, amount: string) => {
      await this.erc20Client.setContractHash(this.contractHash!);
      let keys = jsutils.getKeys("keys");
      const transferDeployHash = await this.erc20Client.transfer(
        keys,
        to,
        amount,
        TRANSFER_PAYMENT_AMOUNT!
      );
      console.log("... Transfer deploy hash: ", transferDeployHash);
    
      let result=await getDeploy(NODE_ADDRESS!, transferDeployHash);
      console.log("... Token transfer successfully");
    }
}

// const test = async () => {

//   await erc20.setContractHash("109ebe63caa9165a85574f41889eaf7285bd11ed0416f085252d6343bac897cf");

//   // // // //name
//   const name = await erc20.name();
//   console.log(`... Contract name: ${name}`);

//   // // //symbol
//   const symbol = await erc20.symbol();
//   console.log(`... Contract symbol: ${symbol}`);

//   //decimal
//   const decimal = await erc20.decimal();
//   console.log(`... Contract decimal: ${decimal}`);

//   //totalsupply
//   let totalSupply = await erc20.totalSupply();
//   console.log(`... Total supply: ${totalSupply}`);

//   // // // // //balanceof
//   let balance = await erc20.balanceOf("574958f9ccd8da547401d68af010074ee3acdc8499d2d1572d95623398f2326e");
//   console.log(`... Balance: ${balance}`);

//   // // //nonce
//   // // let nonce = await erc20.nonce(KEYS.publicKey);
//   // // console.log(`... Nonce: ${nonce}`);

//   // // // //allowance
//   //let allowance = await erc20.allowance(KEYS.publicKey,KEYS.publicKey);
//  // console.log(`... Allowance: ${allowance}`);
 
  

//   //  balanceof
//   // let balance = await erc20.balanceOfcontract(PAIR_CONTRACT!);
//   // console.log(`... Balance: ${balance}`);

//   // // //burn
//   // // const burnDeployHash = awa183.9it erc20.burn(
//   // //   KEYS,
//   // //   KEYS.publicKey,
//   // //   BURN_AMOUNT!,
//   // //   BURN_PAYMENT_AMOUNT!
//   // // );
//   // // console.log("... Burn deploy hash: ", burnDeployHash);

//   // // await getDeploy(NODE_ADDRESS!, burnDeployHash);
//   // // console.log("... Token burned successfully");

//   // // //totalsupply
//   // // totalSupply = await erc20.totalSupply();
//   // // console.log(`... Total supply: ${totalSupply}`);

//   // //approve
//   // const approveDeployHash = await erc20.approve(
//   //   KEYS,
//   //   "278fed58fba4528036a419d45cfb662d2496037bafb174dba2e01fa10d66383e",
//   //   "100000000000",
//   //   APPROVE_PAYMENT_AMOUNT!
//   // );
//   // console.log("... Approve deploy hash: ", approveDeployHash);

//   // await getDeploy(NODE_ADDRESS!, approveDeployHash);
//   // console.log("... Token approved successfully");

//   // // //transfer
//   const transferDeployHash = await erc20.transfer(
//     KEYS,
//     "24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1",
//     TRANSFER_AMOUNT!,
//     TRANSFER_PAYMENT_AMOUNT!
//   );
//   console.log("... Transfer deploy hash: ", transferDeployHash);

//   let result=await getDeploy(NODE_ADDRESS!, transferDeployHash);
//   console.log("... Token transfer successfully");

//   // // //transfer_from
//   // // const transferfromDeployHash = await erc20.transferFrom(
//   // //   KEYS,
//   // //   KEYS.publicKey,
//   // //   KEYS.publicKey,
//   // //   TRANSFER_FROM_AMOUNT!,
//   // //   TRANSFER_FROM_PAYMENT_AMOUNT!
//   // // );
//   // // console.log("... TransferFrom deploy hash: ", transferfromDeployHash);

//   // // await getDeploy(NODE_ADDRESS!, transferfromDeployHash);
//   // // console.log("... Token transfer successfully");

// };

export {ERC20};