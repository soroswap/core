import axios from 'axios';
import { config } from '../utils/env_config.js';
import * as StellarSdk from 'stellar-sdk';

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
  console.log('Public key: ', loadedConfig.admin.publicKey());
  console.log('Horizon RPC', loadedConfig.horizonRpc);
  let source = await loadedConfig.horizonRpc.loadAccount(loadedConfig.admin.publicKey()!);
  console.log('Source: ', source);

  const operation = StellarSdk.Operation.changeTrust({
    source: source.accountId(),
    asset: new StellarSdk.Asset(tokenSymbol, tokenIssuer),
  });
  console.log('Operation: ', operation);

  const txn = new StellarSdk.TransactionBuilder(source, {
    fee: '100',
    timebounds: { minTime: 0, maxTime: 0 },
    networkPassphrase: loadedConfig.passphrase,
  })
    .addOperation(operation)
    .setTimeout(StellarSdk.TimeoutInfinite)
    .build();

  console.log('Transaction: ', txn);

  txn.sign(loadedConfig.admin);

  console.log('\nTransaction Signed: ', txn);

  try {
    let response = await loadedConfig.horizonRpc.submitTransaction(txn);
    return response;
  } catch (error) {
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
    const axiosInstance = axios.create({
      baseURL: 'https://api.soroswap.finance',
      headers: {
        'Cache-Control': 'no-cache',
      },
    });
    const { data } = await axiosInstance.get('/api/tokens');
    tokensList = data.find((item: tokensResponse) => item.network === network).tokens;
  }

  for (const token of tokensList) {
    if (!token.issuer) {
      console.log(`No issuer found for ${token.code}`);
    } else {
      console.log(`Setting trustline for ${token.code}`);
      await setTrustline(token.code, token.issuer);
    }
  }
}

const network = process.argv[2];
const loadedConfig = config(network);

setTrustlines();
