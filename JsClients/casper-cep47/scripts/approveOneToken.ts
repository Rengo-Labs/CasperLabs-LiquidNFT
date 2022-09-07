import {Cep47} from './cep47';
let cep47 = new Cep47();
cep47.approve(process.argv[2]!,[process.argv[3]]!);

/*
    "Script approveOneToken comments": {
        "Description" : "use it to approve one token for a given package-hash",
        "Syntax" : "npm run approveOneToken <package-hash> <tokenId>",
        "Example" : "npm run approveOneToken 7fcc17f692368169ba30ea0e90496c4a95d36d7bc8956e2305a86a38bce44675 15"
    },
*/