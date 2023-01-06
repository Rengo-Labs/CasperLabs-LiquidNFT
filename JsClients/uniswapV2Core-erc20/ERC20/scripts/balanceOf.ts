import {ERC20} from "./erc20";
let erc20 = new ERC20();

erc20.balanceOf(process.argv[2]!);
