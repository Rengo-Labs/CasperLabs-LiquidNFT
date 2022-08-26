import {
  CasperClient,
  CLPublicKey,
  CLByteArray,
  CLValueBuilder,
  CLMap,
  DeployUtil,
  Keys,
  RuntimeArgs,
} from "casper-js-sdk";

import * as utils from "./utils";
import { RecipientType} from "./types";

class LIQUIDNFTClient {
  
  constructor(

    private nodeAddress: string,
    private chainName: string,
    private eventStreamAddress?: string,
    
  ) 
  {}

  public async install(
    keys: Keys.AsymmetricKey,
    defaultCount: string,
    defaultToken: string,
    defaultTarget: string,
    contractName: string,
    paymentAmount: string,
    wasmPath: string
  ) {
    
    const _defaultToken = new CLByteArray(
			Uint8Array.from(Buffer.from(defaultToken, "hex"))
		);
    const _defaultTarget = new CLByteArray(
			Uint8Array.from(Buffer.from(defaultTarget, "hex"))
		);
    const runtimeArgs = RuntimeArgs.fromMap({
      default_count: CLValueBuilder.u256(defaultCount),
      default_token: utils.createRecipientAddress(_defaultToken),
      default_target: utils.createRecipientAddress(_defaultTarget),
      contract_name: CLValueBuilder.string(contractName),
    });

    const deployHash = await installWasmFile({
      chainName: this.chainName,
      paymentAmount,
      nodeAddress: this.nodeAddress,
      keys,
      pathToContract: wasmPath,
      runtimeArgs,
    });

    if (deployHash !== null) {
      return deployHash;
    } else {
      throw Error("Problem with installation");
    }
  }

}

interface IInstallParams {
  nodeAddress: string;
  keys: Keys.AsymmetricKey;
  chainName: string;
  pathToContract: string;
  runtimeArgs: RuntimeArgs;
  paymentAmount: string;
}

const installWasmFile = async ({
  nodeAddress,
  keys,
  chainName,
  pathToContract,
  runtimeArgs,
  paymentAmount,
}: IInstallParams): Promise<string> => {
  const client = new CasperClient(nodeAddress);

  // Set contract installation deploy (unsigned).
  let deploy = DeployUtil.makeDeploy(
    new DeployUtil.DeployParams(
      CLPublicKey.fromHex(keys.publicKey.toHex()),
      chainName
    ),
    DeployUtil.ExecutableDeployItem.newModuleBytes(
      utils.getBinary(pathToContract),
      runtimeArgs
    ),
    DeployUtil.standardPayment(paymentAmount)
  );

  // Sign deploy.
  deploy = client.signDeploy(deploy, keys);

  // Dispatch deploy to node.
  return await client.putDeploy(deploy);
};

export default LIQUIDNFTClient;
