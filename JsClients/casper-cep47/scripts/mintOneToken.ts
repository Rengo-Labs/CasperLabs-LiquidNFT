import {Cep47} from './cep47';
let map = new Map<string,string>().set(process.argv[3],process.argv[4]);
let arrayOfMaps = [map];
let tokenIds = [process.argv[2]];
let cep47 = new Cep47();
cep47.mint(tokenIds,arrayOfMaps);

/*
    "mintOne description"   :   "use it to mint only one NFT with given ID, 
                                fails with user error 3 if id is repeated",
    "mintOne syntax"        :   "npm run mintOneToken <tokenId> <key> <value>",
    "mintOne example"       :   "npm run mintOneToken 13 name AwesomeNFT"
*/

