import {Cep47} from './cep47';
let cep47 = new Cep47();
cep47.approve(process.argv[2],[process.argv[3]]);

/*
    "Script approveOneToken comments": {
        "Description" : "use it to approve one token for a given package-hash",
        "Syntax" : "npm run approveOneToken <package-hash> <tokenId>",
        "Example" : "npm run approveOneToken 841f21fefb97f759a555255b0a414fda519c6deba4007c3a17ada7dc233552d8 0"
    },
*/