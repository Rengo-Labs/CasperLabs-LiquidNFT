import {Cep47} from './cep47';
let cep47 = new Cep47();
cep47.approve(process.argv[2]!,[process.argv[3]]!);

/*
    "Script approveOneToken comments": {
        "Description" : "use it to approve one token for a given package-hash",
        "Syntax" : "npm run approveOneToken <package-hash> <tokenId>",
        "Example" : "npm run approveOneToken f7e686e9086b54918896bda93b490d878abf9a4c35006f68f8fb6ce8811cdff0 18"
    },
*/