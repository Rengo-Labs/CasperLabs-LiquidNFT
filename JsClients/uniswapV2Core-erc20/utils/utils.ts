import {
    Keys,
} from "casper-js-sdk";

import * as fs from 'fs';

export const getKeys = (keyPath: string) => {  
    let keys = Keys.Ed25519.parseKeyFiles(
    `${keyPath}/public_key.pem`,
    `${keyPath}/secret_key.pem`);
    return keys;
}

export const getDeploymentCount = () => {
    return fs.readFileSync('deploymentCount','utf8');
  }
  
export const updateDeploymentCount = () => {
    let val:bigint = BigInt(fs.readFileSync('deploymentCount','utf8'));
    let newVal = val + BigInt(1);
    fs.writeFileSync('deploymentCount',newVal.toString(),{encoding:'utf8',flag:'w'});
  }

