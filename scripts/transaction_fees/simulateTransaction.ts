import * as StellarSdk from "stellar-sdk";
import * as fs from 'fs';

export const getCurrentTimePlusOneHour = (): number => {
    // Get the current time in milliseconds
    const now = Date.now();
  
    // Add one hour (3600000 milliseconds)
    const oneHourLater = now + 36000000;
  
    const oneHourLaterSeconds = Math.floor(oneHourLater / 1000);
    return oneHourLaterSeconds;
};

// Function to read the content of a file
function readFromFile(filePath: string): string {
    try {
      return fs.readFileSync(filePath, 'utf-8').trim();
    } catch (error) {
      console.error(`Error reading file ${filePath}: ${error}`);
      process.exit(1);
    }
}

// Read public key from file
const publicKeyFilePath = '/workspace/.soroban/token_admin_address';
const publicKeyString = readFromFile(publicKeyFilePath);

// Read private key from file
const secretKeyFilePath = '/workspace/.soroban/token_admin_secret';
const secretKeyString = readFromFile(secretKeyFilePath);

// Read router contract address from file
const contractIdFilePath = '/workspace/.soroban/router_id';
const contractIdString = readFromFile(contractIdFilePath);


console.log("Public Key:", publicKeyString);
console.log("Secret Key:", secretKeyString);
console.log("Contract ID:", contractIdString);

async function main() {
    const contractId = contractIdString;
    const routerContract = new StellarSdk.Contract(contractId);
    
    // Right now, this is just the default fee for this example.
    const fee = StellarSdk.BASE_FEE;
    const args = {
        path: ["CCYUZYBB6WRHVZ4WVXK6S46CHHKRNIT4TEU2VEHGNJUHLV2G7CXGUC47", "CAE5562QRYL4XIC5IVMRITBY764DGT4IWCS3MTPM537OQVOAM6AUG2UZ"],
        amountIn: "2000",
        amountOutMin: "0",
        to: {
            publicKey: publicKeyString
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
    const sourceSecretKey = secretKeyString;
    const sourceKeypair = StellarSdk.Keypair.fromSecret(sourceSecretKey);
    const simulatedTransaction = await server.simulateTransaction(preparedTransaction);

    
    console.log("simulatedTransaction.minResourceFee:");
    type simulatedTransactionKey = keyof typeof simulatedTransaction;
    const minResourceFeeVar = 'minResourceFee' as simulatedTransactionKey;
    const minResourceFee = simulatedTransaction[minResourceFeeVar];
    console.log("minResourceFee:", minResourceFee);
    

    // preparedTransaction.sign(sourceKeypair);
    
    // server.sendTransaction(transaction).then(result => {
    //   console.log("hash:", result.hash);
    //   console.log("status:", result.status);
    //   console.log("result:", result);
    // });
}

main();