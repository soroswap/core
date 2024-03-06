import { Asset } from 'stellar-sdk';
import { deployStellarAsset } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { TokensBook } from '../utils/tokens_book.js';

const network = process.argv[2];
const loadedConfig = config(network);

export async function setupNativeToken(tokensBook: TokensBook) {
  try {
    console.log('-------------------------------------------------------');
    console.log('Adding XLM to tokens book');
    console.log('-------------------------------------------------------');
    const xlmToken = {
      address: Asset.native().contractId(loadedConfig.passphrase),
      name: 'Stellar Lumens',
      logoURI: 'https://assets.coingecko.com/coins/images/100/standard/Stellar_symbol_black_RGB.png',
      symbol: 'XLM',
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
