import {LiquidNFT} from "./lnft";
let liquidNFT = new LiquidNFT();
let tokenIds = [process.argv[2]];
liquidNFT.initialize(
    tokenIds,
    process.argv[3]!,
    process.argv[4]!,
    process.argv[5]!,
    process.argv[6]!,
    process.argv[7]!,
    process.argv[8]!,);

/*
    "Script initialize comments": {
        "Description" : "use it to initialize locker",
        "Syntax" : "npm run initialize <tokenId> <cep47PackageHash> <tokenOwnerAccountHash> <floorAsked> <totalAsked> <paymentTime> <paymentRate>",
        "Example" : "npm run initialize 1 737588742efd608e68a1ae1bde3955d61e1d3f72b0e85f7755efe2f14363b943 24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1 1000000000 5000000000 86400000 100"
    },
*/