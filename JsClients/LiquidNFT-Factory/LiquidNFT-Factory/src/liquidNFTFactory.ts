import {
  CasperClient,
  CLPublicKey,
  CLByteArray,
  CLValueBuilder,
  CLMap,
  DeployUtil,
  Keys,
  RuntimeArgs,
  CLAccountHash,
} from "casper-js-sdk";
import { RecipientType } from "./types";

import * as utils from "./utils";

class LIQUIDNFTFactoryClientForDeployment {

  constructor(

    private nodeAddress: string,
    private chainName: string,
    private eventStreamAddress?: string,

  ) { }

  public async install(
    keys: Keys.AsymmetricKey,
    defaultToken: string,
    trusteeMultisig: string,
    contractName: string,
    paymentAmount: string,
    wasmPath: string
  ) {

    const _defaultToken = new CLByteArray(
      Uint8Array.from(Buffer.from(defaultToken, "hex"))
    );
    const _trusteeMultisig = new CLAccountHash(Buffer.from(trusteeMultisig, "hex"));
    const runtimeArgs = RuntimeArgs.fromMap({
      default_token: utils.createRecipientAddress(_defaultToken),
      trustee_multisig: utils.createRecipientAddress(_trusteeMultisig),
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

export default LIQUIDNFTFactoryClientForDeployment;