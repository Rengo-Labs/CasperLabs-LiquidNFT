
# Casper-cep47

## Requirement

Make sure you have created and funded the keys before testing.

## Commands

1. Run ```make all`` command.
2. Run ```npm ci``` command in JsClient Folder to install the node packages.
3. Run ```npm run install``` to deploy contracts to test-net.
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
    "install": "ts-node scripts/install.ts",
    "test:installed": "ts-node scripts/installed.ts",
    "mintOneToken": "ts-node scripts/mintOneToken.ts",
    "approveOneToken": "ts-node scripts/approveOneToken.ts",
    "balanceOf": "ts-node scripts/balanceOf.ts"
  },
```

Use the following commands to perform testing
```
npm run mintOneToken <tokenId> <key> <value>
npm run approveToken <package-hash> <tokenId>
npm run balanceOf <public key pem file path> <secret key pem file path>

```

CONFIGURE .env BEFORE TESTING

#### Note: .env file is in JSClient folder

go to js client folder
run command npm ci
Copy keys folder to casper-cep47 folder OR generate key using keygen(if using keygen funds account)

