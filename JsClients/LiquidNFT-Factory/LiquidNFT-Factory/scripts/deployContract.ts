import {deployContract} from "./lnftFactory";
deployContract(process.argv[2]!);

/*
    "Script deployContract comments": {
        "Description" : "use it to deploy Liquid NFT Factory",
        "Syntax" : "npm run deployContract <defaultToken>",
        "Example" : "npm run deployContract 56b77636b3af55977cfeea4eb22d18394ee2aa2ba4b2afe3a13a91adc26a1222"
    },
*/

//ERC20 Contract Hash = hash-267c011232d1c8d113474b20047f2aabd8d0a704ca9b34dad62cd8372c7a4790
//ERC20 Package Hash = hash-56b77636b3af55977cfeea4eb22d18394ee2aa2ba4b2afe3a13a91adc26a1222