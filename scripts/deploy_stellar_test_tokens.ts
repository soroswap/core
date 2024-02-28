import { Asset } from 'stellar-sdk';
import { deployStellarAsset } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { Token, TokensBook } from '../utils/tokens_book.js';

const network = process.argv[2];
const loadedConfig = config(network);

const name_parts = [
  "zim", "lay", "veo", "tak", "rud", "pia", "nov", "kul", "jor", "fyx",
  "bax", "wun", "voe", "quy", "pyr", "otz", "mil", "kra", "jix", "gex",
  "dex", "uxi", "tro", "siv", "rya", "nef", "laz", "kev", "jam", "fiz",
  "cyo", "vax", "uvi", "tez", "rog", "peq", "nyl", "lom", "kib", "jah"
];

function generateRandomName() {
  const part1 = name_parts[Math.floor(Math.random() * name_parts.length)];
  const part2 = name_parts[Math.floor(Math.random() * name_parts.length)];
  return part1 + part2;
}

export async function deployStellarTestTokens(numberOfTokens: number, resetTokensBook: boolean, tokensBook: TokensBook) {

  try {
    if (resetTokensBook) {
      tokensBook.resetNetworkTokens(network);
    }

    for (let i = 0; i < numberOfTokens; i++) {
      const name = generateRandomName();
      const symbol = name.substring(0, 4).toUpperCase();
      const asset = new Asset(symbol, loadedConfig.admin.publicKey());
      const contractId = asset.contractId(loadedConfig.passphrase);
      const result = await deployStellarAsset(asset, loadedConfig.admin);

      const newToken: Token = {
        address: contractId,
        name: `${asset.code}:${asset.issuer}`,
        logoURI: '',
        symbol: symbol,
        decimals: 7,
      }
  
      if (result.status === 'SUCCESS') {
        tokensBook.addToken(network, newToken);
        console.log(`Token ${symbol} deployed successfully with contractId: ${contractId}!`);
      }
    }
    tokensBook.writeToFile();
  } catch (error) {
    console.log('🚀 « error:', error);
    
  }
}