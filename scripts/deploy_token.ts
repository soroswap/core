import { Address, Contract, Keypair, nativeToScVal } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';
import { deploySorobanToken } from '../utils/contract.js';
import { Token } from '../utils/tokens_book.js';
import { invoke } from '../utils/tx.js';

/**
 * Deploy a token contract and initialize it
 * @param name Name of the token
 * @param symbol Symbol of the token
 * @param addressBook AddressBook instance
 * @param source Keypair of the source account
 */
export async function deployToken(
  name: string,
  symbol: string,
  logoURI: string,
  source: Keypair,
  addressBook: AddressBook,
) {
  try {
    const contractId = await deploySorobanToken('token', addressBook, source);

    // Initializing Token
    const tokenInitParams = [
      new Address(source.publicKey()).toScVal(),
      nativeToScVal(7, { type: 'u32' }),
      nativeToScVal(name, { type: 'string' }),
      nativeToScVal(symbol, { type: 'string' }),
    ];

    const contractInstance = new Contract(contractId!);
    const contractOperation = contractInstance.call('initialize', ...tokenInitParams);
    const result = await invoke(contractOperation, source, false);

    const newToken: Token = {
      name: name,
      contract: contractId!,
      code: symbol,
      logoURI: logoURI,
      decimals: 7,
    }

    if (result.status === 'SUCCESS') {
      return newToken
    } else {
      throw Error (`Token ${symbol} deployment failed with contractId: ${contractId}!`);
    }
  } catch (error) {
    console.log('🚀 « error:', error);
  }
}
