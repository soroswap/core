import { Account, Keypair, SorobanRpc, Transaction, TransactionBuilder, xdr } from 'stellar-sdk';
import { config } from './env_config.js';

type txResponse = SorobanRpc.Api.SendTransactionResponse | SorobanRpc.Api.GetTransactionResponse;
type txStatus = SorobanRpc.Api.SendTransactionStatus | SorobanRpc.Api.GetTransactionStatus;

const network = process.argv[2];
const loadedConfig = config(network);

export async function signWithKeypair(
  txXdr: string,
  passphrase: string,
  source: Keypair
): Promise<string> {
  const tx = new Transaction(txXdr, passphrase);
  tx.sign(source);
  return tx.toXDR();
}

// export async function logInvocation(invocation: Promise<ContractResult<any>>) {
//   console.log('invoking contract...');
//   const result = await invocation;
//   console.log('Hash: ', result.hash);
//   console.log(JSON.stringify(result.resources, null, 2));
//   console.log(result.toString());
//   result.unwrap();
//   console.log();
// }

export async function invoke(
  operation: string | xdr.Operation,
  source: Keypair,
  sim: boolean
): Promise<any> {
  const txBuilder = await createTxBuilder(source);
  if (typeof operation === 'string') {
    operation = xdr.Operation.fromXDR(operation, 'base64');
  }
  txBuilder.addOperation(operation);
  const tx = txBuilder.build();
  return invokeTransaction(tx, source, sim);
}

export async function invokeTransaction(tx: Transaction, source: Keypair, sim: boolean) {
  // simulate the TX
  const simulation_resp = await loadedConfig.rpc.simulateTransaction(tx);
  if (SorobanRpc.Api.isSimulationError(simulation_resp)) {
    // No resource estimation available from a simulation error. Allow the response formatter
    // to fetch the error.
    return simulation_resp;
  } else if (sim) {
    // Only simulate the TX. Assemble the TX to borrow the resource estimation algorithm in
    // `assembleTransaction` and return the simulation results.
    const prepped_tx = SorobanRpc.assembleTransaction(tx, simulation_resp).build();
    return prepped_tx;
  }

  // assemble and sign the TX
  const txResources = simulation_resp.transactionData.build().resources();
  simulation_resp.minResourceFee = (Number(simulation_resp.minResourceFee) + 10000000).toString();
  const sim_tx_data = simulation_resp.transactionData
    .setResources(
      txResources.instructions() == 0 ? 0 : txResources.instructions() + 500000,
      txResources.readBytes(),
      txResources.writeBytes()
    )
    .build();
  const assemble_tx = SorobanRpc.assembleTransaction(tx, simulation_resp);
  sim_tx_data.resourceFee(
    xdr.Int64.fromString((Number(sim_tx_data.resourceFee().toString()) + 100000).toString())
  );
  const prepped_tx = assemble_tx.setSorobanData(sim_tx_data).build();
  prepped_tx.sign(source);
  const tx_hash = prepped_tx.hash().toString('hex');

  console.log('submitting tx...');
  let response: txResponse = await loadedConfig.rpc.sendTransaction(prepped_tx);
  let status: txStatus = response.status;
  console.log(`Hash: ${tx_hash}`);
  // Poll this until the status is not "NOT_FOUND"
  while (status === 'PENDING' || status === 'NOT_FOUND') {
    // See if the transaction is complete
    await new Promise((resolve) => setTimeout(resolve, 2000));
    console.log('checking tx...');
    response = await loadedConfig.rpc.getTransaction(tx_hash);
    status = response.status;
  }
  return response;
}

export async function createTxBuilder(source: Keypair): Promise<TransactionBuilder> {
  try {
    const account: Account = await loadedConfig.rpc.getAccount(source.publicKey());
    return new TransactionBuilder(account, {
      fee: '10000',
      timebounds: { minTime: 0, maxTime: 0 },
      networkPassphrase: loadedConfig.passphrase,
    });
  } catch (e: any) {
    console.error(e);
    throw Error('unable to create txBuilder');
  }
}

// export async function invokeClassicOp(operation: xdr.Operation<Operation>, source: Keypair) {
//   console.log('invoking classic op...');
//   const txBuilder = await createTxBuilder(source);
//   txBuilder.addOperation(operation);
//   const tx = txBuilder.build();
//   tx.sign(source);
//   try {
//     let response: txResponse = await config.rpc.sendTransaction(tx);
//     let status: txStatus = response.status;
//     const tx_hash = response.hash;
//     console.log(`Hash: ${tx_hash}\n`);
//     // Poll this until the status is not "NOT_FOUND"
//     while (status === 'PENDING' || status === 'NOT_FOUND') {
//       // See if the transaction is complete
//       await new Promise((resolve) => setTimeout(resolve, 2000));
//       console.log('checking tx...');
//       response = await config.rpc.getTransaction(tx_hash);
//       status = response.status;
//     }
//     console.log('Transaction status:', response.status);
//     if (status === 'ERROR') {
//       console.log(response);
//     }
//   } catch (e) {
//     console.error(e);
//     throw Error('failed to submit classic op TX');
//   }
// }
