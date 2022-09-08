import {deployContract} from "./lnftFactory";
deployContract(process.argv[2]!);

/*
    "Script deployContract comments": {
        "Description" : "use it to deploy Liquid NFT Factory",
        "Syntax" : "npm run deployContract <defaultToken>",
        "Example" : "npm run deployContract 4efb977f33caaddc15ebd244a1245b3e930cc9fc898b98792688ba7ecfad4c04"
    },
*/
