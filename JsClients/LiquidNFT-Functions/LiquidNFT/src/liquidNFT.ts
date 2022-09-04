import {
  CasperClient,
  CLPublicKey,
  CLAccountHash,
  CLByteArray,
  CLKey,
  CLString,
  CLTypeBuilder,
  CLValue,
  CLValueBuilder,
  CLValueParsers,
  CLMap,
  DeployUtil,
  EventName,
  EventStream,
  Keys,
  RuntimeArgs,
  CLList,
  CLU256,
  encodeBase16
} from "casper-js-sdk";
import { Some, None } from "ts-results";
import * as blake from "blakejs";
import { concat } from "@ethersproject/bytes";
import { LIQUIDNFTEvents } from "./constants";
import * as utils from "./utils";
import { RecipientType, IPendingDeploy } from "./types";
import {createRecipientAddress } from "./utils";
const serialize = require('serialize-javascript');


function deserialize(serializedJavascript){  
    return eval('(' + serializedJavascript + ')');
}

class LIQUIDNFTClient {
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

  private isListening = false;
  private pendingDeploys: IPendingDeploy[] = [];

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

 

//NEW FACTORY FUNCTIONS

public async  updateMaster(
  keys: Keys.AsymmetricKey,
  newMaster: RecipientType,
  paymentAmount: string
) {
  
  const runtimeArgs = RuntimeArgs.fromMap({
    new_master: utils.createRecipientAddress(newMaster),
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
  //paymentToken: RecipientType, 
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




  
  public async  initialize(
    keys: Keys.AsymmetricKey,
    ids: string[],
    tokenAddress: string,
    tokenOwner: string,
    //tokenOwner: RecipientType,
    floorAsked: string,
    totalAsked: string,
    paymentTime: string,
    paymentRate: string,
    paymentAmount: string
  ) {
    
    const _tokenAddress = new CLByteArray(
			Uint8Array.from(Buffer.from(tokenAddress, "hex"))
		);
    const _tokenOwner = new CLByteArray(
			Uint8Array.from(Buffer.from(tokenOwner, "hex"))
		);
    let list = [];
    for (let i = 0; i < 5; i++) {
      const p = new CLU256(0);
      list.push(p);
    }
    const runtimeArgs = RuntimeArgs.fromMap({
      //token_id: new CLList(list),
      token_id: CLValueBuilder.list(ids.map(id => CLValueBuilder.u256(id))),
      token_address: utils.createRecipientAddress(_tokenAddress),
     /// token_owner: utils.createRecipientAddress(_tokenOwner),
      token_owner: new CLKey(_tokenOwner),
      //token_owner: utils.createRecipientAddress(tokenOwner),
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
    //refundAddress: string,
    refundAddress: RecipientType,
    paymentAmount: string
  ) {
    
    // const _refund_address = new CLByteArray(
		// 	Uint8Array.from(Buffer.from(refundAddress, "hex"))
		// );

    const runtimeArgs = RuntimeArgs.fromMap({
      //refund_address: utils.createRecipientAddress(_refund_address),
      refund_address: utils.createRecipientAddress(refundAddress),
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
    //refundAddress: RecipientType,
    paymentAmount: string
  ) {
    
    const _refund_address = new CLByteArray(
			Uint8Array.from(Buffer.from(refundAddress, "hex"))
		);

    const runtimeArgs = RuntimeArgs.fromMap({
       refund_address: utils.createRecipientAddress(_refund_address),
      //refund_address: utils.createRecipientAddress(refundAddress),
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
    //tokenHolder: RecipientType,
    tokenHolder: string,
    paymentAmount: string
  ) {

    const _tokenHolder = new CLByteArray(
      	Uint8Array.from(Buffer.from(tokenHolder, "hex"))
      );
    
    const runtimeArgs = RuntimeArgs.fromMap({
      token_amount: CLValueBuilder.u256(tokenAmount),
      //token_holder: utils.createRecipientAddress(tokenHolder),
     token_holder: utils.createRecipientAddress(_tokenHolder),
     //token_holder: new CLKey(tokenHolder),
      
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

  public onEvent(
    eventNames: LIQUIDNFTEvents[],
    callback: (
      eventName: LIQUIDNFTEvents,
      deployStatus: {
        deployHash: string;
        success: boolean;
        error: string | null;
      },
      result: any | null
    ) => void
  ): any {
    if (!this.eventStreamAddress) {
      throw Error("Please set eventStreamAddress before!");
    }
    if (this.isListening) {
      throw Error(
        "Only one event listener can be create at a time. Remove the previous one and start new."
      );
    }
    const es = new EventStream(this.eventStreamAddress);
    this.isListening = true;

    es.subscribe(EventName.DeployProcessed, (value: any) => {
      const deployHash = value.body.DeployProcessed.deploy_hash;

      const pendingDeploy = this.pendingDeploys.find(
        (pending) => pending.deployHash === deployHash
      );

      if (!pendingDeploy) {
        return;
      }

      if (
        !value.body.DeployProcessed.execution_result.Success &&
        value.body.DeployProcessed.execution_result.Failure
      ) {
        callback(
          pendingDeploy.deployType,
          {
            deployHash,
            error:
              value.body.DeployProcessed.execution_result.Failure.error_message,
            success: false,
          },
          null
        );
      } else {
        const { transforms } =
          value.body.DeployProcessed.execution_result.Success.effect;

        const ERC20Events = transforms.reduce((acc: any, val: any) => {
          if (
            val.transform.hasOwnProperty("WriteCLValue") &&
            typeof val.transform.WriteCLValue.parsed === "object" &&
            val.transform.WriteCLValue.parsed !== null
          ) {
            const maybeCLValue = CLValueParsers.fromJSON(
              val.transform.WriteCLValue
            );
            const clValue = maybeCLValue.unwrap();
            if (clValue && clValue instanceof CLMap) {
              const hash = clValue.get(
                CLValueBuilder.string("contract_package_hash")
              );
              const event = clValue.get(CLValueBuilder.string("event_type"));
              if (
                hash &&
                // NOTE: Calling toLowerCase() because current JS-SDK doesn't support checksumed hashes and returns all lower case value
                // Remove it after updating SDK
                hash.value() === this.contractPackageHash.toLowerCase() &&
                event &&
                eventNames.includes(event.value())
              ) {
                acc = [...acc, { name: event.value(), clValue }];
              }
            }
          }
          return acc;
        }, []);

        ERC20Events.forEach((d: any) =>
          callback(
            d.name,
            { deployHash, error: null, success: true },
            d.clValue
          )
        );
      }

      this.pendingDeploys = this.pendingDeploys.filter(
        (pending) => pending.deployHash !== deployHash
      );
    });
    es.start();

    return {
      stopListening: () => {
        es.unsubscribe(EventName.DeployProcessed);
        es.stop();
        this.isListening = false;
        this.pendingDeploys = [];
      },
    };
  }

  public addPendingDeploy(deployType: LIQUIDNFTEvents, deployHash: string) {
    this.pendingDeploys = [...this.pendingDeploys, { deployHash, deployType }];
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

const toCLMap = (map: Map<string, string>) => {
  const clMap = CLValueBuilder.map([
    CLTypeBuilder.string(),
    CLTypeBuilder.string(),
  ]);
  for (const [key, value] of Array.from(map.entries())) {
    clMap.set(CLValueBuilder.string(key), CLValueBuilder.string(value));
  }
  return clMap;
};

const fromCLMap = (map: Map<CLString, CLString>) => {
  const jsMap = new Map();
  for (const [key, value] of Array.from(map.entries())) {
    jsMap.set(key.value(), value.value());
  }
  return jsMap;
};

export default LIQUIDNFTClient;
