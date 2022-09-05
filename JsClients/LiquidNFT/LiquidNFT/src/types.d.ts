import { CLAccountHash, CLByteArray, CLPublicKey } from "casper-js-sdk";

export type RecipientType = CLPublicKey | CLAccountHash | CLByteArray;


import {LIQUIDNFTEvents} from "./constants";

export interface IPendingDeploy {
  deployHash: string;
  deployType: LIQUIDNFTEvents;
}
