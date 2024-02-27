import { Address, SorobanRpc, nativeToScVal } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';
import { airdropAccount, bumpContractCode, bumpContractInstance, deployContract, installContract, invokeContract } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { signWithKeypair } from '../utils/tx.js';
import { deploySorobanTestTokens } from './deploy_soroban_test_tokens.js';

export async function deployAndInitContracts(addressBook: AddressBook) {
  const signWithAdmin = (txXdr: string) =>
    signWithKeypair(txXdr, rpc_network.passphrase, loadedConfig.admin);
  
  await airdropAccount(loadedConfig.admin);
  console.log('-------------------------------------------------------');
  console.log('Installing Soroswap Contracts');
  // Soroswap Pair
  console.log('-------------------------------------------------------');
  await installContract('pair', addressBook, loadedConfig.admin);
  await bumpContractCode('pair', addressBook, loadedConfig.admin);
  // Soroswap Factory
  console.log('-------------------------------------------------------');
  await installContract('factory', addressBook, loadedConfig.admin);
  await bumpContractCode('factory', addressBook, loadedConfig.admin);
  // Soroswap Router
  console.log('-------------------------------------------------------');
  await installContract('router', addressBook, loadedConfig.admin);
  await bumpContractCode('router', addressBook, loadedConfig.admin);

  console.log('-------------------------------------------------------');
  console.log('Deploying and Initializing Soroswap Factory');
  console.log('-------------------------------------------------------');
  await deployContract('factory', 'factory', addressBook, loadedConfig.admin);
  await bumpContractInstance('factory', addressBook, loadedConfig.admin);
  
  // Initializing Soroswap Factory
  const factoryInitParams = [
    new Address(loadedConfig.admin.publicKey()).toScVal(),
    nativeToScVal(Buffer.from(addressBook.getWasmHash('pair'), 'hex')),
  ];
  await invokeContract('factory', addressBook, 'initialize', factoryInitParams, loadedConfig.admin);

  console.log('-------------------------------------------------------');
  console.log('Deploying and Initializing Soroswap Router');
  console.log('-------------------------------------------------------');
  await deployContract('router', 'router', addressBook, loadedConfig.admin);
  await bumpContractInstance('router', addressBook, loadedConfig.admin);

  // Initializing Soroswap Router
  const routerInitParams = [
    new Address(addressBook.getContractId('factory')).toScVal(),
  ];
  await invokeContract('router', addressBook, 'initialize', routerInitParams, loadedConfig.admin);

  if (network != 'mainnet') {
    console.log('-------------------------------------------------------');
    console.log('Deploying new test tokens');
    console.log('-------------------------------------------------------');
    await deploySorobanTestTokens(8, true);
    // Should create liquidity pools with the test tokens...



    
    // await tryDeployStellarAsset(
    //   addressBook,
    //   loadedConfig.admin,
    //   new Asset('wBTC', loadedConfig.admin.publicKey())
    // );
    // await bumpContractInstance('wBTC', addressBook, loadedConfig.admin);
  }
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);

interface RpcNetwork {
  rpc: SorobanRpc.Server;
  passphrase: string;
  opts: { allowHttp: boolean };
}
const rpc_network: RpcNetwork = {
  rpc: loadedConfig.rpc.serverURL.toString(),
  passphrase: loadedConfig.passphrase,
  opts: { allowHttp: true },
};

await deployAndInitContracts(addressBook);
addressBook.writeToFile();