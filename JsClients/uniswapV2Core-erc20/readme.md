# Casperlabs-UniswapV2Core-erc20

## Requirement

Make sure you have created and funded the keys before testing.

## Commands

1. Run ```make all`` command.
2. Run ```npm ci``` command in JsClient Folder to install the node packages.
3. Run ```npm run deploy``` to deploy contracts to test-net.
4. if you encounter this error then try again 
5. Must have atleast 500 CSPR for the deployment.
  ```
  type: 'system',
  errno: 'ETIMEDOUT',
  code: 'ETIMEDOUT'
  ```
5. On

Use the script file in package.json to perform the testing
```
"scripts": {
    "transfer": "ts-node ERC20/scripts/transfer.ts",
    "mint": "ts-node ERC20/scripts/mint.ts",
    "approve": "ts-node ERC20/scripts/approve.ts",
    "deploy": "ts-node ERC20/scripts/deploy.ts",
    "balanceOf": "ts-node ERC20/scripts/balanceOf.ts",
  },
```

Use the following commands to perform testing
```
npm run approve <packageHash>  <amountToApprove>
npm run mint <keyPath>  <amountTomint>

```

CONFIGURE .env BEFORE TESTING

#### Note: .env file is in JSClient folder

go to js client folder
run command npm ci
Copy keys folder to ERC20 folder OR generate key using keygen(if using keygen funds account)

