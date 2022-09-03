import { config } from "dotenv";
config();
import { CEP47Client, utils, constants } from "../src";
import { parseTokenMeta, sleep, getDeploy } from "./utils";

import {
  CLValueBuilder,
  Keys,
  CLPublicKey,
  CLAccountHash,
  CLPublicKeyType,
} from "casper-js-sdk";

const { CEP47Events } = constants;

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  WASM_PATH,
  MASTER_KEY_PAIR_PATH,
  RECEIVER_ACCOUNT_ONE,
  TOKEN_NAME,
  TOKEN_SYMBOL,
  CONTRACT_HASH,
  INSTALL_PAYMENT_AMOUNT,
  MINT_ONE_PAYMENT_AMOUNT,
  MINT_COPIES_PAYMENT_AMOUNT,
  BURN_ONE_PAYMENT_AMOUNT,
  MINT_ONE_META_SIZE,
  MINT_COPIES_META_SIZE,
  MINT_COPIES_COUNT,
  MINT_MANY_META_SIZE,
  MINT_MANY_META_COUNT,
  CONTRACT_NAME
} = process.env;

const TOKEN_META = new Map(parseTokenMeta(process.env.TOKEN_META!));

const KEYS = Keys.Ed25519.parseKeyFiles(
  `${MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

const test = async () => {
  const cep47 = new CEP47Client(
    NODE_ADDRESS!,
    CHAIN_NAME!,
    EVENT_STREAM_ADDRESS!
  );


  // // We don't need hash- prefix so i'm removing it
   await cep47.setContractHash('191add2109008ffc3ffb5034864340d1f12e28ca41882c3fc874ff78a825f514');

  // const name = await cep47.name();
  // console.log(`... Contract name: ${name}`);

  // const symbol = await cep47.symbol();
  // console.log(`... Contract symbol: ${symbol}`);

  // const meta = await cep47.meta();
  // console.log(`... Contract meta: ${JSON.stringify(meta)}`);

  // let totalSupply = await cep47.totalSupply();
  // console.log(`... Total supply: ${totalSupply}`);

  // const mintDeployHash = await cep47.mintOne(
  //   KEYS,
  //   KEYS.publicKey,
  //   //'7c03afbdf6ee6f78e22b124575772b3e94b9927eeb33dd8a1253beb6a310d25a',
  //   null,
  //   new Map([["name", "jan"]]),
  //   MINT_ONE_PAYMENT_AMOUNT!,
  //   //900000
  // );
  // console.log("... Mint deploy hash: ", mintDeployHash);

  // await getDeploy(NODE_ADDRESS!, mintDeployHash);
  // console.log("... Token minted successfully");

  // let tokensOf = await cep47.getTokensOf(KEYS.publicKey);
  // console.log(`Tokens of faucet account`, tokensOf);

  // totalSupply = await cep47.totalSupply();
  // console.log(`... Total supply: ${totalSupply}`);

  // tokensOf = await cep47.getTokensOf(KEYS.publicKey);
  // console.log(tokensOf);

  // let issuerOfToken = await cep47.getIssuerOf(tokensOf[0]);
  // console.log(`... Issuer of token ${tokensOf[0]} is ${issuerOfToken}`);

  // const mintManyDeployHash = await cep47.mintMany(
  //   KEYS,
  //   KEYS.publicKey,
  //   [
  //     new Map([["name", "one"]]),
  //     new Map([["name", "two"]]),
  //     new Map([["name", "three"]]),
  //     new Map([["name", "four"]]),
  //     new Map([["name", "five"]]),
  //   ],
  //   null,
  //   MINT_COPIES_PAYMENT_AMOUNT!
  // );
  // console.log("... Mint Many deploy hash: ", mintManyDeployHash);

  // await getDeploy(NODE_ADDRESS!, mintManyDeployHash);
  // console.log("... Many tokens minted successfully");

  // totalSupply = await cep47.totalSupply();
  // console.log(`... Total supply: ${totalSupply}`);

  // const mintCopiesDeployHash = await cep47.mintCopies(
  //   KEYS,
  //   KEYS.publicKey,
  //   new Map([["name", "copied"]]),
  //   ["A6", "A7", "A8", "A9", "A10"],
  //   5,
  //   MINT_COPIES_PAYMENT_AMOUNT!
  // );
  // console.log("... Mint Copies deploy hash: ", mintCopiesDeployHash);

  // await getDeploy(NODE_ADDRESS!, mintCopiesDeployHash);
  // console.log("... Copy tokens minted successfully");

  // //mint
  // const mintDeployHash = await cep47.mint(
  //   //'7c03afbdf6ee6f78e22b124575772b3e94b9927eeb33dd8a1253beb6a310d25a',
  //   KEYS.publicKey,
  //   null,
  //   new Map([["name", "copied"]]),
  //   MINT_ONE_PAYMENT_AMOUNT!,
  //   KEYS.publicKey,
  //   KEYS,
  // );
  // console.log("... Mint deploy hash: ", mintDeployHash);

  // await getDeploy(NODE_ADDRESS!, mintDeployHash);
  // console.log("... Token minted successfully.");



  //mint
  // const mintDeployHash = await cep47.mint(
  //   KEYS,
  //   //"c1957f3b89a76399480b9d8914ecc90edc879fa7e40f6df0eabfb9eef66316bc",
  //   KEYS.publicKey,
  //   ["81","91"],
  //   [new Map([['TOKEN-61', 'Metadata for token-61']]),new Map([['TOKEN-71', 'Metadata for token-71']])],
  //   MINT_ONE_PAYMENT_AMOUNT!
  // );
  // console.log("... Mint deploy hash: ", mintDeployHash);
  // await getDeploy(NODE_ADDRESS!, mintDeployHash);
  // console.log("... Token minted successfully.");

  //aprove
   const approveDeployHash = await cep47.approve(
    KEYS,
    //PACKAGE_HASH!,
    '841f21fefb97f759a555255b0a414fda519c6deba4007c3a17ada7dc233552d8',
    ["81","91"],
    MINT_ONE_PAYMENT_AMOUNT!
  );
  console.log("... Approve deploy hash: ", approveDeployHash);

  await getDeploy(NODE_ADDRESS!, approveDeployHash);
  console.log("... Token approved successfully");


  // let balance = await cep47.balanceOf(KEYS.publicKey);
  // console.log(`... Balance of account ${KEYS.publicKey.toAccountHashStr()}`);
  // console.log(`... Balance: ${balance}`);

  // tokensOf = await cep47.getTokensOf(KEYS.publicKey);
  // console.log(`... Tokens of  ${KEYS.publicKey.toAccountHashStr()}`);
  // console.log(`... Tokens: ${JSON.stringify(tokensOf, null, 2)}`);

  // const tokenOneId = tokensOf[0]; 

  // let ownerOfTokenOne = await cep47.getOwnerOf(tokenOneId);
  // console.log(`... Owner of token: ${tokenOneId}`);
  // console.log(`... Owner: ${ownerOfTokenOne}`);

  // let tokenOneMetadata = await cep47.getTokenMeta(tokenOneId);
  // console.log(`... Metadata of token: ${tokenOneId}`);
  // console.log(`... Metadata: `);
  // console.log(tokenOneMetadata);

  // const newTokenOneMetadata = new Map([
  //   ["color", "red"],
  //   ["flavour", "vanilla"],
  // ]);
  // let updatedTokenMetaDeployHash = await cep47.updateTokenMetadata(
  //   KEYS,
  //   tokenOneId,
  //   newTokenOneMetadata,
  //   MINT_ONE_PAYMENT_AMOUNT!
  // );
  // console.log(
  //   "... Update token metadata deploy hash: ",
  //   updatedTokenMetaDeployHash
  // );
  // await getDeploy(NODE_ADDRESS!, updatedTokenMetaDeployHash);
  // console.log("... Token metadata updated sucessfully");

  // tokenOneMetadata = await cep47.getTokenMeta(tokenOneId);
  // console.log(`... Metadata of token: ${tokenOneId}`);
  // console.log(`... Metadata: `);
  // console.log(tokenOneMetadata);

  // totalSupply = await cep47.totalSupply();
  // console.log(`... Total supply: ${totalSupply}`);

  // const burnTokenOneDeployHash = await cep47.burnOne(
  //   KEYS,
  //   new CLAccountHash(KEYS.publicKey.toAccountHash()),
  //   tokenOneId,
  //   BURN_ONE_PAYMENT_AMOUNT!
  // );
  // console.log("... Burn one deploy hash: ", burnTokenOneDeployHash);
  // await getDeploy(NODE_ADDRESS!, burnTokenOneDeployHash);
  // console.log("... Token burnt successfully");

  // totalSupply = await cep47.totalSupply();
  // console.log(`... Total supply: ${totalSupply}`);

  // tokensOf = await cep47.getTokensOf(KEYS.publicKey);
  // let listOfTokensToBurn = tokensOf.map((t: any) => t).slice(0, 3);

  // const burnManyTokensDeployHash = await cep47.burnMany(
  //   KEYS,
  //   new CLAccountHash(KEYS.publicKey.toAccountHash()),
  //   listOfTokensToBurn,
  //   String(parseInt(BURN_ONE_PAYMENT_AMOUNT!) * listOfTokensToBurn.length)
  // );
  // console.log("... Burn many deploy hash: ", burnManyTokensDeployHash);
  // await getDeploy(NODE_ADDRESS!, burnManyTokensDeployHash);
  // console.log("... Many tokens burnt successfully");

  // totalSupply = await cep47.totalSupply();
  // console.log(`... Total supply: ${totalSupply}`);

  // const receiverAccount = CLPublicKey.fromHex(RECEIVER_ACCOUNT_ONE!);

  // tokensOf = await cep47.getTokensOf(KEYS.publicKey);

  // const tokensToTransfer = tokensOf.map((t: any) => t).slice(0, 2);

  // const transferManyDeployHash = await cep47.transferManyTokens(
  //   KEYS,
  //   receiverAccount,
  //   tokensToTransfer,
  //   MINT_COPIES_PAYMENT_AMOUNT!
  // );
  // console.log(
  //   `... Transfer of ${
  //     tokensToTransfer.length
  //   } tokens to account: ${receiverAccount.toAccountHashStr()}`
  // );
  // console.log("... Transfer Many deploy hash: ", transferManyDeployHash);

  // await getDeploy(NODE_ADDRESS!, transferManyDeployHash);
  // console.log("Transfer Many successfull");

  // // let tokensOfAccountOne = await cep47.getTokensOf(receiverAccount);
  // console.log(`... Tokens of  ${receiverAccount.toAccountHashStr()}`);
  // console.log(`... Tokens: ${JSON.stringify(tokensOfAccountOne, null, 2)}`);
};

test();

const mintToken = async () => {
  const cep47 = new CEP47Client(
    NODE_ADDRESS!,
    CHAIN_NAME!,
    EVENT_STREAM_ADDRESS!
  );

  await cep47.setContractHash('191add2109008ffc3ffb5034864340d1f12e28ca41882c3fc874ff78a825f514');

   const mintDeployHash = await cep47.mint(
    KEYS,
    //"c1957f3b89a76399480b9d8914ecc90edc879fa7e40f6df0eabfb9eef66316bc",
    KEYS.publicKey,
    ["81","91"],
    [new Map([['TOKEN-61', 'Metadata for token-61']]),new Map([['TOKEN-71', 'Metadata for token-71']])],
    MINT_ONE_PAYMENT_AMOUNT!
  );
  console.log("... Mint deploy hash: ", mintDeployHash);
  await getDeploy(NODE_ADDRESS!, mintDeployHash);
  console.log("... Token minted successfully.");

}

const approveToken = async () => {
  const cep47 = new CEP47Client(
    NODE_ADDRESS!,
    CHAIN_NAME!,
    EVENT_STREAM_ADDRESS!
  );

  await cep47.setContractHash('191add2109008ffc3ffb5034864340d1f12e28ca41882c3fc874ff78a825f514');

  const approveDeployHash = await cep47.approve(
    KEYS,
    //PACKAGE_HASH!,
    '841f21fefb97f759a555255b0a414fda519c6deba4007c3a17ada7dc233552d8',
    ["81","91"],
    MINT_ONE_PAYMENT_AMOUNT!
  );
  console.log("... Approve deploy hash: ", approveDeployHash);

  await getDeploy(NODE_ADDRESS!, approveDeployHash);
  console.log("... Token approved successfully");

}

export{mintToken,approveToken};