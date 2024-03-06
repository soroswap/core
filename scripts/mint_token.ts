import { Address, Keypair, nativeToScVal, xdr } from 'stellar-sdk';
import { invokeCustomContract } from '../utils/contract.js';
import { config } from '../utils/env_config.js';

const network = process.argv[2];
const loadedConfig = config(network);

export async function mintToken(contractId: string, amount: number, to: string, source: Keypair) {
  try {
    const mintTokensParams: xdr.ScVal[] = [
      new Address(to).toScVal(),
      nativeToScVal(amount, { type: 'i128' }),
    ]

    return await invokeCustomContract(contractId, 'mint', mintTokensParams, source);
  } catch (error) {
    console.log('🚀 « error:', error);
    
  }
}