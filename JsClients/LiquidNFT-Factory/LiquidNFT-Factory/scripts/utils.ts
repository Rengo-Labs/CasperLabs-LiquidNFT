import { CasperClient } from "casper-js-sdk";
import {
  Keys,
} from "casper-js-sdk";

import * as fs from 'fs';

export const sleep = (ms: number) => {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export const getDeploy = async (NODE_URL: string, deployHash: string) => {
  const client = new CasperClient(NODE_URL);
  let i = 300;
  while (i != 0) {
    //console.log("i: ",i);
      const [deploy, raw] = await client.getDeploy(deployHash);
      if (raw.execution_results.length !== 0){
          // @ts-ignore
          if (raw.execution_results[0].result.Success) {
           
              return [deploy.header.timestamp,raw.execution_results[0].block_hash];
          } else {
              // @ts-ignore
              throw Error("Contract execution: " + raw.execution_results[0].result.Failure.error_message);
          }
      } else {
          i--;
          await sleep(1000);
          continue;
      }
  }
  throw Error('Timeout after ' + i + 's. Something\'s wrong');
}



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

