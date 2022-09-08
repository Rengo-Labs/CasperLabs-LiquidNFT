import {ERC20} from "./erc20";
let erc20 = new ERC20();

erc20.mint(process.argv[2]!,process.argv[3]!);

/*
    "Script mint comments": {
        "Description" : "use it to mint tokens",
        "Syntax" : "npm run mint <keyPath>  <amountTomint>",
        "Example" : "npm run mint ERC20/keys  1000000000000"
    },
*/