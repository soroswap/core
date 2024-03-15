import { Asset } from 'stellar-sdk';
import { deployStellarAsset } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { Token, TokensBook } from '../utils/tokens_book.js';

const network = process.argv[2];
const loadedConfig = config(network);

export async function setupNativeToken(tokensBook: TokensBook) {
  try {
    console.log('-------------------------------------------------------');
    console.log('Adding XLM to tokens book');
    console.log('-------------------------------------------------------');
    const xlmToken: Token  = {
      name: 'Stellar Lumens',
      contract: Asset.native().contractId(loadedConfig.passphrase),
      code: 'XLM',
      icon: 'https://assets.coingecko.com/coins/images/100/standard/Stellar_symbol_black_RGB.png',
      decimals: 7,
    }
    
    tokensBook.prependToken(network, xlmToken);
    tokensBook.writeToFile();
    
    if (network !== 'mainnet') {
      await deployStellarAsset(Asset.native(), loadedConfig.admin)
    }
  } catch (error) {
    console.log('😧 XLM is probably already deployed on this network');
  }
}
