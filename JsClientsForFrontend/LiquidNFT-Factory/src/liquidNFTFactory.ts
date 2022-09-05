import {
  CasperClient,
  CLAccountHash,
  CLByteArray,
  CLKey,
  CLValue,
  CLValueBuilder,
  DeployUtil,
  Keys,
  RuntimeArgs,
  encodeBase16
} from "casper-js-sdk";

import * as utils from "./utils";
const serialize = require('serialize-javascript');

function deserialize(serializedJavascript:string){  
    return eval('(' + serializedJavascript + ')');
}

class LIQUIDNFTFACTORYClientForFunctions {
  private contractName: string = "LIQUIDNFTFACTORY";
  private contractHash: string= "LIQUIDNFTFACTORY";
  private contractPackageHash: string= "LIQUIDNFTFACTORY";
  private namedKeys: {
    balances:string
    metadata: string;
    nonces: string;
    allowances: string;
    ownedTokens: string;
    owners: string;
    paused: string;
    
  };

  constructor(

    private nodeAddress: string,
    private chainName: string,
    private eventStreamAddress?: string,
    
  ) 
  {
    this.namedKeys= {
      balances:"null",
      metadata: "null",
      nonces: "null",
      allowances: "null",
      ownedTokens: "null",
      owners: "null",
      paused: "null"
    }; 
  }

  public async setContractHash(hash: string) {
    const stateRootHash = await utils.getStateRootHash(this.nodeAddress);
    const contractData = await utils.getContractData(
      this.nodeAddress,
      stateRootHash,
      hash
    );

    const { contractPackageHash, namedKeys } = contractData.Contract!;
    this.contractHash = hash;
    this.contractPackageHash = contractPackageHash.replace(
      "contract-package-wasm",
      ""
    );
    const LIST_OF_NAMED_KEYS = [
      'balances',
      'nonces',
      'allowances',
      `${this.contractName}_package_hash`,
      `${this.contractName}_package_hash_wrapped`,
      `${this.contractName}_contract_hash`,
      `${this.contractName}_contract_hash_wrapped`,
      `${this.contractName}_package_access_token`,
    ];
    // @ts-ignore
    this.namedKeys = namedKeys.reduce((acc, val) => {
      if (LIST_OF_NAMED_KEYS.includes(val.name)) {
        return { ...acc, [utils.camelCased(val.name)]: val.key };
      }
      return acc;
    }, {});
  }

  public async result() {
    const result = await contractSimpleGetter(
      this.nodeAddress,
      this.contractHash,
      ["result"]
    );
    const serializedResult = serialize({obj: result.value()});
    let deserializedResult= (deserialize(serializedResult)).obj;
    const propertyValues:number[]= Object.values(deserializedResult.data);
    let convertedValue=encodeBase16(Uint8Array.from(propertyValues));
    return convertedValue;
  }

  public async  updateMaster(
    keys: Keys.AsymmetricKey,
    newMaster: string,
    paymentAmount: string
  ) {

    const _newMaster=new CLKey(new CLAccountHash(Uint8Array.from(Buffer.from(newMaster, "hex"))));
    const runtimeArgs = RuntimeArgs.fromMap({
      new_master: _newMaster
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "update_master",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async  revokeMaster(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "revoke_master",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async  createLiquidLockerJsClient(
    keys: Keys.AsymmetricKey,
    ids: string[],
    tokenAddress: string,
    floorAsked: string,
    totalAsked: string,
    paymentTime: string,
    paymentRate: string,
    paymentToken: string,
    paymentAmount: string,
  ) {
    
    const _tokenAddress = new CLByteArray(
      Uint8Array.from(Buffer.from(tokenAddress, "hex"))
    );
    const _paymentToken = new CLByteArray(
      Uint8Array.from(Buffer.from(paymentToken, "hex"))
    );
    const runtimeArgs = RuntimeArgs.fromMap({
      token_id: CLValueBuilder.list(ids.map(id => CLValueBuilder.u256(id))),
      token_address: utils.createRecipientAddress(_tokenAddress),
      floor_asked: CLValueBuilder.u256(floorAsked),
      total_asked: CLValueBuilder.u256(totalAsked), 
      payment_time: CLValueBuilder.u256(paymentTime),
      payment_rate: CLValueBuilder.u256(paymentRate),
      payment_token: new CLKey(_paymentToken),
      //paymentToken: utils.createRecipientAddress(paymentToken),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "create_liquid_locker_js_client",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async  createEmptyLockerJsClient(
    keys: Keys.AsymmetricKey,
    paymentToken: string,
    //paymentToken: RecipientType, 
    paymentAmount: string
  ) {
    
    const _paymentToken = new CLByteArray(
      Uint8Array.from(Buffer.from(paymentToken, "hex"))
    );
    const runtimeArgs = RuntimeArgs.fromMap({
      payment_token: new CLKey(_paymentToken),
      //paymentToken: utils.createRecipientAddress(paymentToken),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "create_empty_locker_js_client",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async  contributeToLockerJsClient(
    keys: Keys.AsymmetricKey,
    lockersAddress: string,
    //lockersAddress: RecipientType,
    paymentAmountArgs: string, 
    paymentAmount: string
  ) {
    
    const _lockersAddress = new CLByteArray(
      Uint8Array.from(Buffer.from(lockersAddress, "hex"))
    );
    const runtimeArgs = RuntimeArgs.fromMap({
      lockers_address: new CLKey(_lockersAddress),
      //lockers_address: utils.createRecipientAddress(lockersAddress),
      payment_amount: CLValueBuilder.u256(paymentAmountArgs), 
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "contribute_to_locker_js_client",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async  donateToLocker(
    keys: Keys.AsymmetricKey,
    lockersAddress: string,
    //lockersAddress: RecipientType,
    donationAmount: string, 
    paymentAmount: string
  ) {
    
    const _lockersAddress = new CLByteArray(
      Uint8Array.from(Buffer.from(lockersAddress, "hex"))
    );
    const runtimeArgs = RuntimeArgs.fromMap({
      lockers_address: new CLKey(_lockersAddress),
      //lockers_address: utils.createRecipientAddress(lockersAddress),
      donation_amount: CLValueBuilder.u256(donationAmount), 
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "donate_to_locker",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async  paybackToLocker(
    keys: Keys.AsymmetricKey,
    lockersAddress: string,
    //lockersAddress: RecipientType,
    paymentAmountArgs: string, 
    paymentAmount: string
  ) {
    
    const _lockersAddress = new CLByteArray(
      Uint8Array.from(Buffer.from(lockersAddress, "hex"))
    );
    const runtimeArgs = RuntimeArgs.fromMap({
      lockers_address: new CLKey(_lockersAddress),
      //lockers_address: utils.createRecipientAddress(lockersAddress),
      payment_amount: CLValueBuilder.u256(paymentAmountArgs), 
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "payback_to_locker",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

}

interface IContractCallParams {
  nodeAddress: string;
  keys: Keys.AsymmetricKey;
  chainName: string;
  entryPoint: string;
  runtimeArgs: RuntimeArgs;
  paymentAmount: string;
  contractHash: string;
}

const contractCall = async ({
  nodeAddress,
  keys,
  chainName,
  contractHash,
  entryPoint,
  runtimeArgs,
  paymentAmount,
}: IContractCallParams) => {
  const client = new CasperClient(nodeAddress);
  const contractHashAsByteArray = utils.contractHashToByteArray(contractHash);

  let deploy = DeployUtil.makeDeploy(
    new DeployUtil.DeployParams(keys.publicKey, chainName),
    DeployUtil.ExecutableDeployItem.newStoredContractByHash(
      contractHashAsByteArray,
      entryPoint,
      runtimeArgs
    ),
    DeployUtil.standardPayment(paymentAmount)
  );

  // Sign deploy.
  deploy = client.signDeploy(deploy, keys);

  // Dispatch deploy to node.
  const deployHash = await client.putDeploy(deploy);

  return deployHash;
};

const contractSimpleGetter = async (
  nodeAddress: string,
  contractHash: string,
  key: string[]
) => {
  const stateRootHash = await utils.getStateRootHash(nodeAddress);
  const clValue = await utils.getContractData(
    nodeAddress,
    stateRootHash,
    contractHash,
    key
  );

  if (clValue && clValue.CLValue instanceof CLValue) {
    return clValue.CLValue!;
  } else {
    throw Error("Invalid stored value");
  }
};


export default LIQUIDNFTFACTORYClientForFunctions;
