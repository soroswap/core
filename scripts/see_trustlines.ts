import * as StellarSdk from '@stellar/stellar-sdk';
import axios from 'axios';
import { config } from '../utils/env_config.js';
import { TokensBook } from '../utils/tokens_book.js';


const network = process.argv[2];
if (!network) {
  console.log('Please provide a network as argument. Eg: testnet, standalone');
  process.exit(1);
}
const folder = process.argv[3];
const loadedConfig = config(network);
async function main () {
    const publicKey = StellarSdk.Keypair.fromSecret(
        process.env.TRUSTLINE_WALLET_SECRET_KEY!
      ).publicKey();
    const source = await loadedConfig.horizonRpc.loadAccount(publicKey);

    const tokensList = await getTokenList(network);

    // Check if token in tokensList is already in trustline, by comparing the asset_code and asset_issuer
    for (const token of tokensList) {
        const trustlineExists = source.balances.some((balance) => {
            return (
                (balance.asset_type === 'credit_alphanum4' || 
                balance.asset_type === 'credit_alphanum12')
                &&
                balance.asset_code === token.code &&
                balance.asset_issuer === token.issuer
            );
        });
        console.log(`Trustline for ${token.code} exists: ${trustlineExists}`);
    }
}

async function getTokenList(network: string) {
  let tokensList;
  if (network === 'mainnet') {
    const { data } = await axios.get(
      'https://raw.githubusercontent.com/soroswap/token-list/main/tokenList.json'
    );
    tokensList = data.tokens;
  } else {
    const tokensBook = TokensBook.loadFromFile(folder);
    tokensList = tokensBook.getTokensByNetwork(network);
  }
  return tokensList;
}

main();