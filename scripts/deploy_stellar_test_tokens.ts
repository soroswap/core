import { Asset, Keypair } from 'stellar-sdk';
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

export async function deployStellarTestTokens(numberOfTokens: number, resetTokensBook: boolean, tokensBook: TokensBook, source: Keypair) {
  try {
    if (resetTokensBook) {
      tokensBook.resetNetworkTokens(network);
    }

    for (let i = 0; i < numberOfTokens; i++) {
      const name = generateRandomName();
      console.log("🚀 ~ deployStellarTestTokens ~ name:", name)
      const symbol = name.substring(0, 4).toUpperCase();
      const asset = new Asset(symbol, source.publicKey());
      const contractId = asset.contractId(loadedConfig.passphrase);
      try{
        const result = await deployStellarAsset(asset, source);
        const newToken: Token = {
          name: name,
          contract: contractId,
          code: symbol,
          issuer: asset.issuer,
          logoURI: '',
          decimals: 7,
        }
    
        if (result.status === 'SUCCESS') {
          tokensBook.addToken(network, newToken);
          console.log(`Token ${symbol} deployed successfully with contractId: ${contractId}!`);
        }
      }
      catch (error) {
        console.log("Deploying Stellar Asset failed with error: ", error)
      }

    }
    tokensBook.writeToFile();
  } catch (error) {
    console.log('🚀 « error:', error);
    
  }
}