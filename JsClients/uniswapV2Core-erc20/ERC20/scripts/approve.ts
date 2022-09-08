import {ERC20} from "./erc20";
let erc20 = new ERC20();

erc20.approve(process.argv[2]!,process.argv[3]!);

/*
    "Script approve comments": {
        "Description" : "use it to approve tokens",
        "Syntax" : "npm run approve <packageHash>  <amountToApprove>",
        "Example" : "npm run approve f7e686e9086b54918896bda93b490d878abf9a4c35006f68f8fb6ce8811cdff0  1000000000000"
    },
*/