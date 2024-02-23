// import {
//   BackstopClient,
//   EmitterClient,
//   Network,
//   PoolFactoryClient,
//   PoolInitMeta,
//   TxOptions,
// } from '@blend-capital/blend-sdk';
// import { Asset } from 'stellar-sdk';
// import { CometClient } from '../external/comet.js';
// import { tryDeployStellarAsset } from '../external/token.js';
import { AddressBook } from '../utils/address_book.js';
import { airdropAccount, bumpContractCode, bumpContractInstance, deployContract, installContract } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { signWithKeypair } from '../utils/tx.js';

export async function deployAndInitContracts(addressBook: AddressBook) {
  const signWithAdmin = (txXdr: string) =>
    signWithKeypair(txXdr, rpc_network.passphrase, loadedConfig.admin);
  
  await airdropAccount(loadedConfig.admin);
  console.log('Installing Soroswap Contracts');
  // Soroswap Pair
  await installContract('pair', addressBook, loadedConfig.admin);
  await bumpContractCode('pair', addressBook, loadedConfig.admin);
  // Soroswap Factory
  await installContract('factory', addressBook, loadedConfig.admin);
  await bumpContractCode('factory', addressBook, loadedConfig.admin);
  // Soroswap Router
  await installContract('router', addressBook, loadedConfig.admin);
  await bumpContractCode('router', addressBook, loadedConfig.admin);
  // add any other contracts here router / factory / etc same idea install and bump
  if (network != 'mainnet') {
    // Tokens
    console.log('Installing and deploying: Soroswap Mocked Contracts');
    // await installContract('token', addressBook, loadedConfig.admin);
    // await bumpContractCode('token', addressBook, loadedConfig.admin);
    // await deployContract('token', 'token', addressBook, loadedConfig.admin);
    // await bumpContractInstance('token', addressBook, loadedConfig.admin);
    // console.log('Installing and deploying: Tokens');
    // await tryDeployStellarAsset(addressBook, loadedConfig.admin, Asset.native());
    // await bumpContractInstance('XLM', addressBook, loadedConfig.admin);
    // await tryDeployStellarAsset(
    //   addressBook,
    //   loadedConfig.admin,
    //   new Asset('USDC', loadedConfig.admin.publicKey())
    // );
    // await bumpContractInstance('USDC', addressBook, loadedConfig.admin);
    // await tryDeployStellarAsset(
    //   addressBook,
    //   loadedConfig.admin,
    //   new Asset('BLND', loadedConfig.admin.publicKey())
    // );
    // await bumpContractInstance('BLND', addressBook, loadedConfig.admin);
    // await tryDeployStellarAsset(
    //   addressBook,
    //   loadedConfig.admin,
    //   new Asset('wETH', loadedConfig.admin.publicKey())
    // );
    // await bumpContractInstance('wETH', addressBook, loadedConfig.admin);
    // await tryDeployStellarAsset(
    //   addressBook,
    //   loadedConfig.admin,
    //   new Asset('wBTC', loadedConfig.admin.publicKey())
    // );
    // await bumpContractInstance('wBTC', addressBook, loadedConfig.admin);
  }
  console.log('Deploying and Initializing Soroswap Aggregator');
  await deployContract('factory', 'factory', addressBook, loadedConfig.admin);
  await bumpContractInstance('factory', addressBook, loadedConfig.admin);
  // Should initialize the factory with the pair wasm hash
  // const emitter = new EmitterClient(addressBook.getContractId('emitter'));

  await deployContract('router', 'router', addressBook, loadedConfig.admin);
  await bumpContractInstance('router', addressBook, loadedConfig.admin);
  
  // const poolFactory = new PoolFactoryClient(addressBook.getContractId('poolFactory'));
  // await logInvocation(
  //   emitter.initialize(config.admin.publicKey(), signWithAdmin, rpc_network, tx_options, {
  //     blnd_token: addressBook.getContractId('BLND'),
  //     backstop: addressBook.getContractId('backstop'),
  //     backstop_token: addressBook.getContractId('comet'),
  //   })
  // );
  // await logInvocation(
  //   backstop.initialize(config.admin.publicKey(), signWithAdmin, rpc_network, tx_options, {
  //     backstop_token: addressBook.getContractId('comet'),
  //     emitter: addressBook.getContractId('emitter'),
  //     usdc_token: addressBook.getContractId('USDC'),
  //     blnd_token: addressBook.getContractId('BLND'),
  //     pool_factory: addressBook.getContractId('poolFactory'),
  //     drop_list: new Map(),
  //   })
  // );
  // const poolInitMeta: PoolInitMeta = {
  //   backstop: addressBook.getContractId('backstop'),
  //   blnd_id: addressBook.getContractId('BLND'),
  //   usdc_id: addressBook.getContractId('USDC'),
  //   pool_hash: Buffer.from(addressBook.getWasmHash('lendingPool'), 'hex'),
  // };
  // await logInvocation(
  //   poolFactory.initialize(
  //     config.admin.publicKey(),
  //     signWithAdmin,
  //     rpc_network,
  //     tx_options,
  //     poolInitMeta
  //   )
  // );
  // await bumpContractInstance('backstop', addressBook, config.admin);
  // await bumpContractInstance('emitter', addressBook, config.admin);
  // await bumpContractInstance('poolFactory', addressBook, config.admin);
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);
const rpc_network = {
  rpc: loadedConfig.rpc.serverURL.toString(),
  passphrase: loadedConfig.passphrase,
  opts: { allowHttp: true },
};
// const tx_options: TxOptions = {
//   sim: false,
//   pollingInterval: 2000,
//   timeout: 30000,
//   builderOptions: {
//     fee: '10000',
//     timebounds: {
//       minTime: 0,
//       maxTime: 0,
//     },
//     networkPassphrase: config.passphrase,
//   },
// };
await deployAndInitContracts(addressBook);
// addressBook.writeToFile();
