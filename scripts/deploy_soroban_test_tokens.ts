import { readFileSync } from 'fs';
import path from 'path';
import { Address, Contract, SorobanRpc, nativeToScVal } from 'stellar-sdk';
import { fileURLToPath } from 'url';
import { AddressBook } from '../utils/address_book.js';
import { bumpContractCode, deploySorobanToken, installContract } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { Token, TokensBook } from '../utils/tokens_book.js';
import { invoke } from '../utils/tx.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export async function deploySorobanTestTokens(numberOfTokens: number, resetTokensBook: boolean) {
  const fileName = `../../scripts/token_name_ideas.json`;
  // Instaling token contract
  await installContract('token', addressBook, loadedConfig.admin);
  await bumpContractCode('token', addressBook, loadedConfig.admin);

  try {
    if (resetTokensBook) {
      tokensBook.resetNetworkTokens(network);
    }

    const tokenNameIdeas = readFileSync(path.join(__dirname, fileName));
    const tokenNameIdeasObject = JSON.parse(tokenNameIdeas.toString());
    for (let i = 0; i < numberOfTokens; i++) {
      const token = tokenNameIdeasObject.tokens[i];
      const contractId = await deploySorobanToken('token', addressBook, loadedConfig.admin);
      
      // Initializing Token
      const tokenInitParams = [
        new Address(loadedConfig.admin.publicKey()).toScVal(),
        nativeToScVal(Number(7), {type: 'u32'}),
        nativeToScVal(String(token.name), { type: 'string' }),
        nativeToScVal(String(token.symbol), { type: 'string' }),
      ];
      
      const contractInstance = new Contract(contractId!);
      const contractOperation = contractInstance.call('initialize', ...tokenInitParams);
      const result = await invoke(
        contractOperation,
        loadedConfig.admin,
        false,
      );

      const newToken: Token = {
        address: contractId!,
        name: token.name,
        logoURI: token.logoURI,
        symbol: token.symbol,
        decimals: 7,
      }
  
      if (result.status === 'SUCCESS') {
        tokensBook.addToken(network, newToken);
        console.log(`Token ${token.symbol} deployed successfully with contractId: ${contractId}!`);
      }
    }
    tokensBook.writeToFile();
  } catch (error) {
    console.log('🚀 « error:', error);
    
  }
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const tokensBook = TokensBook.loadFromFile();

const loadedConfig = config(network);

interface RpcNetwork {
  rpc: SorobanRpc.Server;
  passphrase: string;
  opts: { allowHttp: boolean };
}
