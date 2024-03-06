import axios from "axios";
import fs from "fs";
import * as path from 'path';
import * as sdk from 'stellar-sdk';
import { ApiErrorResponse, testAccount, tokenContract, tokensFile } from './types';

export const colors = {
  red: '\x1b[31m%s\x1b[0m',
  yellow: '\x1b[33m%s\x1b[0m',
  green: '\x1b[32m%s\x1b[0m',
  cyan: '\x1b[36m%s\x1b[0m',
  purple: '\x1b[35;1m%s\x1b[0m',
}

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

/**
 * Loads tokens from a file based on the specified network.
 * @param network - The network to load tokens from ('standalone' or 'testnet').
 * @returns An array of tokens if the file exists, otherwise undefined.
 */
export function loadTokens(network: string): tokensFile | undefined {
  let filepath: string;
  switch (network) {
    case 'standalone':
      filepath = path.join('/workspace/', './.soroban', 'tokens.json');
      if (fs.existsSync(filepath)) {
        const data = fs.readFileSync(filepath, 'utf8');
        return JSON.parse(data) as tokensFile;
      } else {
        console.error(colors.red, 'No tokens file found.');
        return undefined;
      }
 
    case 'testnet':
      filepath = path.join('/workspace/', './public', 'tokens.json');
      if (fs.existsSync(filepath)) {
        const data = fs.readFileSync(filepath, 'utf8');
        return JSON.parse(data) as tokensFile;
      } else {
        console.error(colors.red, 'No tokens file found.');
        return undefined;
      }

    default:
      console.log(colors.yellow, "Please pass your selected network as an argument")
      console.log(colors.yellow, "Usage: ts-node all.ts standalone|testnet")
      return undefined
  }
}


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
