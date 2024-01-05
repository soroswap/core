import * as sdk from "stellar-sdk";

export interface testAccount {
  privateKey: string;
  publicKey: string;
}

export interface establishPoolTrustlineAndAddLiquidityArgs {
  assetA: sdk.Asset,
  assetB: sdk.Asset,
  source: testAccount,
  amountA?: string,
  amountB?: string
}

export interface deployStellarAssetContractArgs {
  asset: sdk.Asset;
  source: testAccount;
}

export interface ApiErrorResponse {
  extras: {
    result_codes: {
      // Define the properties of result_codes here
      // For example:
      transaction: string;
      operations: string[];
    };
  };
  // Include any other properties that might be in the error response
}

export interface addLiquiditySoroswapArgs {
  tokenA: string;
  tokenB: string;
  amountADesired: string; 
  amountBDesired: string;
  amountAMin: string;
  amountBMin: string;
  source: testAccount;
  to: testAccount;
}

export interface removeLiquiditySoroswapArgs {
  tokenA: string;
  tokenB: string;
  liquidity: string; 
  amountAMin: string;
  amountBMin: string;
  source: testAccount;
  to: testAccount;
}

export interface initializeTokenContractArgs {
  source: testAccount;
  contractId: string;
  name: string;
  symbol: string;
}

export interface liquidityPoolWithdrawArgs {
  poolAsset: sdk.LiquidityPoolAsset,
  source: testAccount,
  amount: string,
  minAmountA: string,
  minAmountB: string
}

export interface paymentArgs {
  from: testAccount;
  to: string;
  amount: string;
  asset: sdk.Asset
}

export interface issueAndDistributeAssetArgs {
  name: string;
  issuer: testAccount;
  destination?: testAccount[]
}

export interface mintTokensArgs {
  source: testAccount;
  contractId: string;
  amount: string;
  destination:string;
}

export interface assetToWrap {
  code: string;
  issuer: string;
}

export interface tokenContract {
  contractId: string;
  address: string;
  name: string;  
  symbol: string;
}

export interface token {
  name: string;
  symbol: string;
  address: string;
  decimals: number;
  logoURI: string;
}