import { Address, SorobanRpc, nativeToScVal } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';
import { airdropAccount, bumpContractCode, bumpContractInstance, deployContract, installContract, invokeContract } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { TokensBook } from '../utils/tokens_book.js';
import { signWithKeypair } from '../utils/tx.js';
import { deployRandomTokens } from './deploy_random_tokens.js';
import { deploySorobanTestTokens } from './deploy_soroban_test_tokens.js';
import { deployStellarTestTokens } from './deploy_stellar_test_tokens.js';
import { multiAddLiquidity } from './multi_add_liquidity.js';
import { setupNativeToken } from './setup_native_token.js';

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
    console.log('Deploying Soroban test tokens');
    console.log('-------------------------------------------------------');
    await deploySorobanTestTokens(8, true, tokensBook, addressBook);
    console.log('-------------------------------------------------------');
    console.log('Adding Liquidity to multiple paths');
    console.log('-------------------------------------------------------');
    await multiAddLiquidity(3, tokensBook, addressBook);
    console.log('-------------------------------------------------------');
    console.log('Deploying Stellar Test Tokens');
    console.log('-------------------------------------------------------');
    await deployStellarTestTokens(4, false, tokensBook);
    console.log('-------------------------------------------------------');
    console.log('Deploying Random tokens for testing');
    console.log('-------------------------------------------------------');
    await deployRandomTokens(8, true, addressBook);
  }

  console.log('-------------------------------------------------------');
  console.log('-------------------------------------------------------');
  console.log("Setup Native Token")
  await setupNativeToken(tokensBook);
}

const network = process.argv[2];
console.log("🚀 ~ network:", network)
const addressBook = AddressBook.loadFromFile(network);
console.log("🚀 ~ addressBook:", addressBook)
const tokensBook = TokensBook.loadFromFile();
console.log("🚀 ~ tokensBook:", tokensBook)

const loadedConfig = config(network);
console.log("🚀 ~ loadedConfig:", loadedConfig)

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
