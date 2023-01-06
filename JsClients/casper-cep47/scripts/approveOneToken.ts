import { Cep47 } from './cep47';
let cep47 = new Cep47();
cep47.approve(process.argv[2]!, [process.argv[3]]!);

/*
    "Script approveOneToken comments": {
        "Description" : "use it to approve one token for a given package-hash",
        "Syntax" : "npm run approveOneToken <package-hash> <tokenId>",
        "Example" : "npm run approveOneToken 084a5fe30ed8d73e73975e0fc6223057b4a97f8a428a9ab5e5ec94cf8de71b33 1"
    },
*/