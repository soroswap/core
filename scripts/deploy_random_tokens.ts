import { Asset } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';
import { deployStellarAsset } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { Token, TokensBook } from '../utils/tokens_book.js';
import { deployToken } from './deploy_token.js';

const network = process.argv[2];
const loadedConfig = config(network);
const randomTokensBook = TokensBook.loadFromFile("random_tokens.json");

const name_parts = [
  "ram", "che", "vok", "rim", "rem", "poe", "vol", "tek", "jir", "fox",
  "pet", "qwa", "yie", "muy", "asd", "das", "fel", "ony", "pil", "szx",
  "dox", "web", "flo", "rie", "rai", "est", "jun", "kiv", "ulk", "jaz",
  "zxa", "vbs", "tro", "tra", "lup", "lep", "wek", "pie", "fer", "hel"
];

function generateRandomName() {
  const part1 = name_parts[Math.floor(Math.random() * name_parts.length)];
  const part2 = name_parts[Math.floor(Math.random() * name_parts.length)];
  return part1 + part2;
}

export async function deployRandomTokens(numberOfTokens: number, resetTokensBook: boolean, addressBook: AddressBook) {

  try {
    if (resetTokensBook) {
      randomTokensBook.resetNetworkTokens(network);
    }

    for (let i = 0; i < numberOfTokens; i++) {
      const name = generateRandomName();
      const symbol = name.substring(0, 4).toUpperCase();

      if (i < numberOfTokens / 2) {
        const deployedToken = await deployToken(name, symbol, '', loadedConfig.admin, addressBook);
        randomTokensBook.addToken(network, deployedToken!);
      } else {
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
          randomTokensBook.addToken(network, newToken);
        }
      }
      console.log(`🚀 Token ${symbol} deployed successfully`);
    }
    randomTokensBook.writeToFile();
  } catch (error) {
    console.log('🚀 « error:', error);
    
  }
}