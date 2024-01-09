import * as sdk from '@stellar/stellar-sdk'
import * as path from 'path';
import axios from "axios";
import fs from "fs";
import { testAccount, ApiErrorResponse, tokenContract, token } from './types'
import tokensFile from '../../../../.soroban/tokens.json'

export const colors = {
  red: '\x1b[31m%s\x1b[0m',
  yellow: '\x1b[33m%s\x1b[0m',
  green: '\x1b[32m%s\x1b[0m',
  cyan: '\x1b[36m%s\x1b[0m',
  purple: '\x1b[35;1m%s\x1b[0m',
}
  
/**
 * Array of tokens.
 * @type {Array<token>}
 */
export const tokens = tokensFile[0]?.tokens || [];

/**
 * Generates a user with a random keypair.
 * @returns {testAccount} The generated user with a private and public key.
 */
export const generateUser = (): testAccount => {
  const keypair = sdk.Keypair.random()
  const publicKey = keypair.publicKey()
  const privateKey = keypair.secret()
  return {
    privateKey,
    publicKey
  }
}

/**
 * Saves the provided accounts to a JSON file.
 * @param accounts An array of testAccount objects to be saved.
 */
export function saveAccounts(accounts: testAccount[]): void {
  const dirPath = path.join('/workspace/', './.soroban');
  const filepath = path.join(dirPath, 'testAccounts.json');

  // Create the directory if it doesn't exist
  if (!fs.existsSync(dirPath)) {
    fs.mkdirSync(dirPath, { recursive: true });
  }

  const data = JSON.stringify(accounts, null, 2);
  fs.writeFileSync(filepath, data);
}

/**
 * Saves the token contracts to a JSON file.
 * @param tokens - The token contracts to be saved.
 */
export function saveContracts(tokens: tokenContract[]): void {
  const dirPath = path.join('/workspace/', './.soroban/test');
  const filepath = path.join(dirPath, 'tokenContracts.json');

  // Create the directory if it doesn't exist
  if (!fs.existsSync(dirPath)) {
    fs.mkdirSync(dirPath, { recursive: true });
  }

  const data = JSON.stringify(tokens, null, 2);
  fs.writeFileSync(filepath, data, 'utf8');
}

/**
 * Loads the token contracts from a JSON file.
 * @returns An array of token contracts if the file exists, otherwise undefined.
 */
export function loadContracts(): tokenContract[] | undefined {
  const filepath = path.join('/workspace/', './.soroban/test', 'tokenContracts.json');

  if(fs.existsSync(filepath)) {
    const data = fs.readFileSync(filepath, 'utf8');
    return JSON.parse(data) as tokenContract[];
  } else {
    console.log('No contracts file found.');
    return undefined;
  }
}

/**
 * Loads the accounts from a JSON file.
 * @returns An array of testAccount objects if the file exists, otherwise undefined.
 */
export function loadAccounts(): testAccount[] | undefined {
  const filepath = path.join('/workspace/', './.soroban', 'testAccounts.json');

  if (fs.existsSync(filepath)) {
    const data = fs.readFileSync(filepath, 'utf8');
    return JSON.parse(data) as testAccount[];
  } else {
    console.log('No accounts file found.');
    return undefined;
  }
}


//------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------


/**
 * Retrieves the network passphrase based on the specified network.
 * @param network The network for which the passphrase is requested.
 * @returns The network passphrase.
 * @throws Error if the network is unsupported. Only standalone and testnet are supported.
 */
export function getNetworkPassphrase(network: string): string {
  switch (network) {
    case "testnet":
      return sdk.Networks.TESTNET;
    case "standalone":
      return sdk.Networks.STANDALONE;
    default:
      throw new Error("Unsupported network. Only standalone and testnet are supported.");
  }
}

/**
 * Logs the result codes from an API error response.
 * @param error The error object, which should be an Axios error with a response.
 */
export function showErrorResultCodes(error: any): void {
  if (axios.isAxiosError(error) && error.response) {
    const apiError = error.response.data as ApiErrorResponse;
    if (apiError && apiError.extras && apiError.extras.result_codes) {
      console.log('Result Codes:', apiError.extras.result_codes);
    } else {
      console.log("error:", error)
      console.log('Error does not have the expected format');
    }
  } else {
    console.error('Non-API error occurred:', error);
  }
}

/**
 * Waits for a transaction confirmation on the Soroban network.
 * @param hash The hash of the transaction to wait for confirmation.
 * @param server The Soroban RPC server instance.
 * @returns A promise that resolves to the confirmation object when the transaction is confirmed.
 */
export async function waitForConfirmation(hash: string, server: sdk.SorobanRpc.Server): Promise<sdk.SorobanRpc.Api.GetSuccessfulTransactionResponse|sdk.SorobanRpc.Api.GetFailedTransactionResponse> {
  let confirmation;
  do {
    confirmation = await server.getTransaction(hash);
    if (confirmation.status !== sdk.SorobanRpc.Api.GetTransactionStatus.NOT_FOUND) {
      break;
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
  } while (true);
  return confirmation;
}

/**
 * Retrieves the balance of a specific asset for a given account.
 * @param args An object containing the account and asset information.
 * @param args.account The Horizon account response object.
 * @param args.asset The asset for which the balance is requested.
 * @returns The balance of the specified asset for the account, or undefined if the asset is not found.
 */
export function getAssetBalance(args: { account: sdk.Horizon.AccountResponse, asset: sdk.Asset }): string | undefined {
  const balance = args.account.balances.find((balance) => {
    if (balance.asset_type === "native") {
      return args.asset.isNative();
    } else if (balance.asset_type === "liquidity_pool_shares") {
      // todo: handle liquidity pool shares
      console.log("Liquidity pool shares not supported yet");
      return false;
    } else {
      return balance.asset_code === args.asset.code && balance.asset_issuer === args.asset.issuer;
    }
  });
  return balance?.balance;
}

/**
 * Creates a new liquidity pool asset with the specified assets.
 * @param assetA The first asset of the liquidity pool.
 * @param assetB The second asset of the liquidity pool.
 * @returns A new instance of the `sdk.LiquidityPoolAsset` class representing the created liquidity pool asset.
 */
export function createLiquidityPoolAsset(assetA: sdk.Asset, assetB: sdk.Asset): sdk.LiquidityPoolAsset {
  return new sdk.LiquidityPoolAsset(assetA, assetB, sdk.LiquidityPoolFeeV18);
}

/**
 * Retrieves the ID of a liquidity pool asset.
 * @param liquidityPoolAsset The liquidity pool asset for which the ID is requested.
 * @returns The ID of the liquidity pool asset as a hexadecimal string.
 */
export function getLiquidityPoolId(liquidityPoolAsset: sdk.LiquidityPoolAsset): string {
  return sdk.getLiquidityPoolId("constant_product",
    liquidityPoolAsset.getLiquidityPoolParameters()
  ).toString("hex");
}

/**
 * Converts a hexadecimal string to a byte array.
 * @param hexString The hexadecimal string to convert.
 * @returns The byte array representation of the hexadecimal string.
 * @throws Throws an error if the input string does not have an even number of hex digits.
 */
export function hexToByte(hexString: string): Uint8Array {
  if (hexString.length % 2 !== 0) {
    throw new Error("Must have an even number of hex digits to convert to bytes");
  }
  const numBytes = hexString.length / 2;
  const byteArray = new Uint8Array(numBytes);
  for (let i = 0; i < numBytes; i++) {
    byteArray[i] = parseInt(hexString.substr(i * 2, 2), 16);
  }
  return byteArray;
}

/**
 * Returns the current time plus one hour in seconds.
 * @returns The current time plus one hour in seconds.
 */
export const getCurrentTimePlusOneHour = (): number => {
  // Get the current time in milliseconds
  const now = Date.now();

  // Add one hour (3600000 milliseconds)
  const oneHourLater = now + 36000000;

  const oneHourLaterSeconds = Math.floor(oneHourLater / 1000);
  return oneHourLaterSeconds;
};

/**
 * Retrieves the router contract address for a specific network from a given URI.
 * @param uri The URI of the server.
 * @param network The network for which the router contract address is requested.
 * @returns The router contract address for the specified network, or "error" if an error occurs.
 */
export async function getRouterContractAddress(uri: string, network: string): Promise<string> {
  try {
    console.log('Fetching router contract address...')
    const response = await axios.get(`${uri}/api/router`);
    const data = response.data;
    if (!data) {
      throw new Error('No router contract address found');
    }
    const router = data.find((router: {network: string, router_id: string, router_address:string}) => router.network === network)
    return router.router_address
  } catch (error) {
    console.log("error:", error);
  }
  
  return "error";
}

/**
 * Creates a new asset with the specified name and issuer public key.
 * @param name The name of the asset.
 * @param issuerPublicKey The public key of the asset issuer.
 * @returns A new instance of the `sdk.Asset` class representing the created asset.
 */
export function createAsset(name: string, issuerPublicKey: string): sdk.Asset {
  return new sdk.Asset(name, issuerPublicKey);
}