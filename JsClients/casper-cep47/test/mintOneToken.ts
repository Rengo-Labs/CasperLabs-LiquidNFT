import {mintOneToken} from './installed';
let map = new Map<string,string>().set(process.argv[3],process.argv[4]);
let arrayOfMaps = [map];
let tokenIds = [process.argv[2]];
mintOneToken(tokenIds,arrayOfMaps);

/*
    "mintOne description"   :   "use it to mint only one NFT with given ID, 
                                fails with user error 3 if id is repeated",
    "mintOne syntax"        :   "npm run mintOne <tokenId> <key> <value>",
    "mintOne example"       :   "npm run mintOne 1 name AwesomeNFT"
*/

