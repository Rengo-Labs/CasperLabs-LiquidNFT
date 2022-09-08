import {
  CasperClient,
  CLByteArray,
  CLKey,
  CLString,
  CLTypeBuilder,
  CLValue,
  CLValueBuilder,
  DeployUtil,
  Keys,
  RuntimeArgs,
  CLU256,
  CLAccountHash
} from "casper-js-sdk";
import * as utils from "./utils";
import { RecipientType} from "./types";

class LIQUIDNFTClientForFunctions {
  private contractName: string = "LIQUIDNFT";
  private contractHash: string= "LIQUIDNFT";
  private contractPackageHash: string= "LIQUIDNFT";
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

  public async  initialize(
    keys: Keys.AsymmetricKey,
    ids: string[],
    tokenAddress: string,
    tokenOwner: string,
    floorAsked: string,
    totalAsked: string,
    paymentTime: string,
    paymentRate: string,
    paymentAmount: string
  ) {
    
    const _tokenAddress = new CLByteArray(
			Uint8Array.from(Buffer.from(tokenAddress, "hex"))
		);
    const _tokenOwner =new CLKey(new CLAccountHash(Uint8Array.from(Buffer.from(tokenOwner, "hex"))));

    let list = [];
    for (let i = 0; i < 5; i++) {
      const p = new CLU256(0);
      list.push(p);
    }
    const runtimeArgs = RuntimeArgs.fromMap({
      token_id: CLValueBuilder.list(ids.map(id => CLValueBuilder.u256(id))),
      token_address: utils.createRecipientAddress(_tokenAddress),
      token_owner: _tokenOwner,
      floor_asked: CLValueBuilder.u256(floorAsked),
      total_asked: CLValueBuilder.u256(totalAsked), 
      payment_time: CLValueBuilder.u256(paymentTime),
      payment_rate: CLValueBuilder.u256(paymentRate)
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "initialize",
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

  public async  increasePaymentRate(
    keys: Keys.AsymmetricKey,
    newPaymentRate: string,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
      new_payment_rate: CLValueBuilder.u256(newPaymentRate),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "increase_payment_rate",
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

  public async  decreasePaymentTime(
    keys: Keys.AsymmetricKey,
    newPaymentRate: string,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
      new_payment_rate: CLValueBuilder.u256(newPaymentRate),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "decrease_payment_time",
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

  public async  enableLocker(
    keys: Keys.AsymmetricKey,
    prepayAmount: string,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
      prepay_amount: CLValueBuilder.u256(prepayAmount),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "enable_locker",
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

  public async  disableLocker(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "disable_locker",
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

  public async  rescueLocker(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "rescue_locker",
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

  public async  refundDueDisabled(
    keys: Keys.AsymmetricKey,
    refundAddress: string,
    paymentAmount: string
  ) {

    const _refundAddress =new CLKey(new CLAccountHash(Uint8Array.from(Buffer.from(refundAddress, "hex"))));
    const runtimeArgs = RuntimeArgs.fromMap({
      refund_address: _refundAddress,
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "refund_due_disabled",
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

  public async  refundDueSingle(
    keys: Keys.AsymmetricKey,
    refundAddress: string,
    paymentAmount: string
  ) {
    
    const _refundAddress =new CLKey(new CLAccountHash(Uint8Array.from(Buffer.from(refundAddress, "hex"))));
    const runtimeArgs = RuntimeArgs.fromMap({
      refund_address: _refundAddress,
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "refund_due_single",
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

  public async  donateFunds(
    keys: Keys.AsymmetricKey,
    donationAmount: string,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
      donation_amount: CLValueBuilder.u256(donationAmount),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "donate_funds",
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

  public async  payBackFunds(
    keys: Keys.AsymmetricKey,
    paymentAmountArgs: string,
    paymentAmount: string
  
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
      payment_amount: CLValueBuilder.u256(paymentAmountArgs),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "pay_back_funds",
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

  public async  liquidateLocker(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "liquidate_locker",
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

  public async  claimInterestSingle(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "claim_interest_single",
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

  public async  claimInterestPublic(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "claim_interest_public",
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

  public async  calculateEpoch(
    keys: Keys.AsymmetricKey,
    totalValue: string,
    paymentTime: string,
    paymentRate: string,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
      total_value: CLValueBuilder.u256(totalValue),
      payment_time: CLValueBuilder.u256(paymentTime),
      payment_rate: CLValueBuilder.u256(paymentRate)
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "calculate_epoch",
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

  public async  calculatePaybacks(
    keys: Keys.AsymmetricKey,
    totalValue: string,
    paymentTime: string,
    paymentRate: string,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
      total_value: CLValueBuilder.u256(totalValue),
      payment_time: CLValueBuilder.u256(paymentTime),
      payment_rate: CLValueBuilder.u256(paymentRate)
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "calculate_paybacks",
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

  public async  getLateDays(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "get_late_days",
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

  public async  penaltyAmount(
    keys: Keys.AsymmetricKey,
    totalCollected: string,
    lateDaysAmount: string,
    paymentAmount: string
  ) {
    
    const runtimeArgs = RuntimeArgs.fromMap({
      total_collected: CLValueBuilder.u256(totalCollected),
      late_days_amount: CLValueBuilder.u256(lateDaysAmount),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "penalty_amount",
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

  public async  makeContribution(
    keys: Keys.AsymmetricKey,
    tokenAmount: string,
    tokenHolder: string,
    paymentAmount: string
  ) {

    const _tokenHolder = new CLByteArray(
      	Uint8Array.from(Buffer.from(tokenHolder, "hex"))
      );
    
    const runtimeArgs = RuntimeArgs.fromMap({
      token_amount: CLValueBuilder.u256(tokenAmount),
      token_holder: utils.createRecipientAddress(_tokenHolder),
    });

    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "make_contribution",
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


export default LIQUIDNFTClientForFunctions;
