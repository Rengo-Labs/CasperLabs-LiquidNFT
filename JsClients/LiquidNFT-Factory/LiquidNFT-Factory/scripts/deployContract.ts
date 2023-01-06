import { deployContract } from "./lnftFactory";
deployContract(process.argv[2]!, process.argv[3]!);

/*
    "Script deployContract comments": {
        "Description" : "use it to deploy Liquid NFT Factory",
        "Syntax" : "npm run deployContract <defaultToken> <trusteeMultisig>",
        "Example" : "npm run deployContract eb4f9467e9a3f43cf59b3da7e60f0a6fc99659326fd2054b96fc4c3520b81c6f 24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1"
    },
*/
