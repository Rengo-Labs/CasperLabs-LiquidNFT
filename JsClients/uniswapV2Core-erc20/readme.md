# Casperlabs-UniswapV2Core-jsClient

## Prerequisite

Make sure you have created, pasted and funded the keys before testing.

## Generate the keys

Paste this command on the ubuntu terminal, that will create a keys folder for you containing public key , public key hex and secret key.

```
casper-client keygen keys

```
## Paste the keys

Paste the keys folder created by the above command to ERC20, FACTORY, PAIR, FLASHSWAPPER, LIBRARY and WCSPR folders.

## Fund the key

We can fund the keys from casper live website faucet page on testnet.

Link:

```
https://testnet.cspr.live/tools/faucet

```

## Testing

Use the script file in package.json to perform the testing
```
"scripts": {
    "test:erc20install": "ts-node ERC20/test/install.ts",
    "test:erc20installed": "ts-node ERC20/test/installed.ts",
    "test:pairinstall": "ts-node PAIR/test/install.ts",
    "test:pairinstalled": "ts-node PAIR/test/installed.ts",
    "test:factoryinstall": "ts-node FACTORY/test/install.ts",
    "test:factoryinstalled": "ts-node FACTORY/test/installed.ts",
    "test:wcsprinstall": "ts-node WCSPR/test/install.ts",
    "test:libraryinstall": "ts-node LIBRARY/test/install.ts",
    "test:flashswapperinstall": "ts-node FLASHSWAPPER/test/install.ts",
  },
```

Use the following commands to perform testing
```
npm run test:erc20install
npm run test:erc20installed

npm run test:factoryinstall
npm run test:factoryinstalled

npm run test:pairinstall
npm run test:pairinstalled

npm run test:wcsprinstall

npm run test:libraryinstall

npm run test:flashswapperinstall


```

* CONFIGURE .env BEFORE TESTING

