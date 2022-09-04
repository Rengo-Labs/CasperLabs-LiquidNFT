import {approveToken} from './installed';
approveToken(process.argv[2],[process.argv[3]]);

/*
    "Script approveToken comments": {
        "Description" : "use it to approve one token for a given package-hash",
        "Syntax" : "npm run approveToken <package-hash> <tokenId>",
        "Example" : "npm run approveToken 841f21fefb97f759a555255b0a414fda519c6deba4007c3a17ada7dc233552d8 0"
    },
*/