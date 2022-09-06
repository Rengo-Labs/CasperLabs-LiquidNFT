import {Cep47} from './cep47';
let cep47 = new Cep47();
cep47.approve(process.argv[2]!,[process.argv[3]]!);

/*
    "Script approveOneToken comments": {
        "Description" : "use it to approve one token for a given package-hash",
        "Syntax" : "npm run approveOneToken <package-hash> <tokenId>",
        "Example" : "npm run approveOneToken 7a803911a6421e92fbe62d355927594d90e70756dfe7c9b0e22fd908ff746929 13"
    },
*/