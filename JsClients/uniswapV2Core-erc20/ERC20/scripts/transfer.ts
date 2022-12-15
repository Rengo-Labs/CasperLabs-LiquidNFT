import {ERC20} from "./erc20";
let erc20 = new ERC20();

erc20.transfer(process.argv[2]!,process.argv[3]);
