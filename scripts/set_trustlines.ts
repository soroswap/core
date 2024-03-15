import axios from 'axios';
import { config } from '../utils/env_config.js';
import * as StellarSdk from 'stellar-sdk';
import { TokensBook } from '../utils/tokens_book.js';

export interface TokenType {
  address: string;
  name: string;
  symbol: string;
  decimals?: number;
  logoURI?: string;
}

export interface tokensResponse {
  network: string;
  tokens: TokenType[];
}

export async function setTrustline(tokenSymbol: string, tokenIssuer: string) {
  const publicKey = StellarSdk.Keypair.fromSecret(
    process.env.TRUSTLINE_WALLET_SECRET_KEY!
  ).publicKey();
  const source = await loadedConfig.horizonRpc.loadAccount(publicKey);

  const operation = StellarSdk.Operation.changeTrust({
    source: source.accountId(),
    asset: new StellarSdk.Asset(tokenSymbol, tokenIssuer),
  });

  const txn = new StellarSdk.TransactionBuilder(source, {
    fee: '100',
    timebounds: { minTime: 0, maxTime: 0 },
    networkPassphrase: loadedConfig.passphrase,
  })
    .addOperation(operation)
    .setTimeout(StellarSdk.TimeoutInfinite)
    .build();

  const keyPair = StellarSdk.Keypair.fromSecret(process.env.TRUSTLINE_WALLET_SECRET_KEY!);
  txn.sign(keyPair);

  try {
    let response = await loadedConfig.horizonRpc.submitTransaction(txn);
    console.log('Trustline set for ', tokenSymbol);
    return response;
  } catch (error: any) {
    console.log(error);
  }
}

export async function setTrustlines() {
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

  for (const token of tokensList) {
    if (!token.issuer) {
      console.log(`No issuer found for ${token.code}, unable to set trustline.\n`);
    } else {
      console.log(`Setting trustline for ${token.code}`);
      await setTrustline(token.code, token.issuer);
    }
  }
}

const network = process.argv[2];
if (!network) {
  console.log('Please provide a network as argument. Eg: testnet, standalone');
  process.exit(1);
}
const folder = process.argv[3];
const loadedConfig = config(network);

setTrustlines();
