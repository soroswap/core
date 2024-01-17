import * as StellarSdk from "stellar-sdk";

export const getCurrentTimePlusOneHour = (): number => {
    // Get the current time in milliseconds
    const now = Date.now();
  
    // Add one hour (3600000 milliseconds)
    const oneHourLater = now + 36000000;
  
    const oneHourLaterSeconds = Math.floor(oneHourLater / 1000);
    return oneHourLaterSeconds;
};

async function main() {
    const contractId = 'CBU3B6DZLJPX3FQNUHVGADIGS4DZZEWMGJC4OJS6NCVP42DWBJ6SW6VW';
    const routerContract = new StellarSdk.Contract(contractId);
    
    // Right now, this is just the default fee for this example.
    const fee = StellarSdk.BASE_FEE;
    const args = {
        path: ["CAXDLYHEKAXUMV5PIH4P4YVXTRNIRYPKHW4ER3DA3QH3ZIW2EAHJIUWI", "CCH7QSH4TVNIRG3K4G5S4G6LYBKWGKW2XKZVSQ45LUY53IF55K7ZOEFG"],
        amountIn: "2000",
        amountOutMin: "0",
        to: {
            publicKey: "GBKM5QKX4AQRSGFJUDB5QZXIZJIZ4XXQ6CHGU336JD4OXDRA4TSYXIPU"
        }
    }
    const path = args.path.map((token) => new StellarSdk.Address(token));
    const scValParams = [
        StellarSdk.nativeToScVal(Number(args.amountIn), { type: "i128" }),
        StellarSdk.nativeToScVal(Number(args.amountOutMin), { type: "i128" }),
        StellarSdk.nativeToScVal(path, { type: "Vec" }),
        new StellarSdk.Address(args.to.publicKey).toScVal(),
        StellarSdk.nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }),
    ];
    
    const op = routerContract.call("swap_exact_tokens_for_tokens", ...scValParams);
    const server = new StellarSdk.SorobanRpc.Server("http://stellar:8000/soroban/rpc", {
        allowHttp: true
    });
    const account = await server.getAccount(args.to.publicKey);
    const transaction = new StellarSdk.TransactionBuilder(account, { fee })
      // Uncomment the following line to build transactions for the live network. Be
      // sure to also change the horizon hostname.
      //.setNetworkPassphrase(StellarSdk.Networks.PUBLIC)
      .setNetworkPassphrase(StellarSdk.Networks.STANDALONE)
      .setTimeout(30) // valid for the next 30s
      // Add an operation to call increment() on the contract
      .addOperation(op)
      .build();
    
    const preparedTransaction = await server.prepareTransaction(transaction);
    
    // Sign this transaction with the secret key
    // NOTE: signing is transaction is network specific. Test network transactions
    // won't work in the public network. To switch networks, use the Network object
    // as explained above (look for StellarSdk.Network).
    const sourceSecretKey = "SAUHH63AQVK6KP4QMF6IILS7SWC7NXULN4X5CJEV763DQ7GY227ZG7JD";
    const sourceKeypair = StellarSdk.Keypair.fromSecret(sourceSecretKey);
    const simulatedTransaction = await server.simulateTransaction(preparedTransaction);

    console.log("simulatedTransaction:");
    console.log(simulatedTransaction);
    
    console.log("simulatedTransaction.transactionData:");
    type simulatedTransactionKey = keyof typeof simulatedTransaction;
    const transactionDataVar = 'transactionData' as simulatedTransactionKey;
    const transactionData = simulatedTransaction[transactionDataVar];
    console.log(transactionData);

    console.log("\nsimulatedTransaction.transactionData._data:")
    type sorobanDataBuilderKey = keyof typeof transactionData;
    const dataVar = '_data' as sorobanDataBuilderKey;
    const _data = transactionData[dataVar]
    console.log(_data);

    console.log("\nsimulatedTransaction.transactionData._data._attributes:")
    type childStructKey = keyof typeof _data;
    const attributesVar = '_attributes' as childStructKey;
    const _attributes = _data[attributesVar];
    console.log(_attributes);

    console.log("\nsimulatedTransaction.transactionData._data._attributes.resourceFee:")
    type attributesKey = keyof typeof _attributes;
    const resourceFeeVar = 'resourceFee' as attributesKey;
    const resourceFee = _attributes[resourceFeeVar];
    console.log(resourceFee);

    console.log("\nsimulatedTransaction.transactionData._data._attributes.resourceFee._value:")
    type resourceFeeKey = keyof typeof resourceFee;
    const valueVar = '_value' as resourceFeeKey;
    const _value = resourceFee[valueVar];
    console.log(_value);

    

    preparedTransaction.sign(sourceKeypair);
    
    // server.sendTransaction(transaction).then(result => {
    //   console.log("hash:", result.hash);
    //   console.log("status:", result.status);
    //   console.log("result:", result);
    // });
}

main();