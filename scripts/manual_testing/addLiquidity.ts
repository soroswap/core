import { Address, nativeToScVal, xdr } from "stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { getTokenBalance, invokeContract } from "../../utils/contract.js";
import { colors } from "../../utils/index.js";
import { Token, TokensBook } from "../../utils/tokens_book.js";
import { getCurrentTimePlusOneHour } from "../../utils/tx.js";

export const addLiquidity = async (network: string, tokensBook: TokensBook, addressBook: AddressBook, loadedConfig: any) => {
  console.log('')
  console.log(colors.purple, '====================')
  console.log(colors.purple, '= ADD_LIQUIDITY.ts =')
  console.log(colors.purple, '====================')
  console.log('')

  const testAccount = loadedConfig.getUser('TESTING_ACCOUNT_SECRET_KEY');

  try {
    const tokens = tokensBook.getTokensByNetwork(network);
    const token0: Token = tokens!.find(token => token.symbol === 'BTC')!;
    const token1: Token = tokens!.find(token => token.symbol === 'EURC')!;
  
    console.log(colors.cyan, "Fetching token balances...")
    const token0FirstBalance = await getTokenBalance(
      token0.address,
      testAccount.publicKey(),
      testAccount,
    )
    const token1FirstBalance = await getTokenBalance(
      token1.address,
      testAccount.publicKey(),
      testAccount,
    )
  
    console.log(colors.green, `${token0.symbol}_Balance:`, token0FirstBalance)
    console.log(colors.green, `${token1.symbol}_Balance:`, token1FirstBalance)
  
    console.log(colors.cyan, "Adding liquidity...")
    const addLiquidityParams: xdr.ScVal[] = [
      new Address(token0.address).toScVal(),
      new Address(token1.address).toScVal(),
      nativeToScVal(25000000000, { type: "i128" }),
      nativeToScVal(25000000000, { type: "i128" }),
      nativeToScVal(0, { type: "i128" }),
      nativeToScVal(0, { type: "i128" }),
      new Address(testAccount.publicKey()).toScVal(),
      nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }),
    ];
  
    await invokeContract(
      "router",
      addressBook,
      "add_liquidity",
      addLiquidityParams,
      testAccount,
    );
  
    console.log(colors.cyan, "Fetching token balances...")
    const token0LastBalance = await getTokenBalance(
      token0.address,
      testAccount.publicKey(),
      testAccount,
    )
    const token1LastBalance = await getTokenBalance(
      token1.address,
      testAccount.publicKey(),
      testAccount,
    )
    console.log(colors.green, `${token0.symbol}_Balance:`, token0LastBalance)
    console.log(colors.green, `${token1.symbol}_Balance:`, token1LastBalance)
    console.log(colors.green, '- Done. -')
  } catch (error) {
    console.log('😩 > Error Adding Liquidity:', error);
  }
}