import { Address, nativeToScVal, xdr } from "stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { getTokenBalance, invokeContract } from "../../utils/contract.js";
import { colors } from "../../utils/index.js";
import { Token, TokensBook } from "../../utils/tokens_book.js";
import { getCurrentTimePlusOneHour } from "../../utils/tx.js";

export const swap = async (network: string, tokensBook: TokensBook, addressBook: AddressBook, loadedConfig: any) => {
  console.log('')
  console.log(colors.purple, '===========')
  console.log(colors.purple, '= SWAP.ts =')
  console.log(colors.purple, '===========')
  console.log('')

  const testAccount = loadedConfig.getUser('TESTING_ACCOUNT_SECRET_KEY');
  
  try {
    const tokens = tokensBook.getTokensByNetwork(network);
    const token0: Token = tokens![1];
    const token1: Token = tokens![7];
  
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
  
    console.log(colors.cyan, "Swapping tokens...")
  
    const path = [new Address(token0.address), new Address(token1.address)]
    const swapParams: xdr.ScVal[] = [
      nativeToScVal(2500000, { type: "i128" }),
      nativeToScVal(0, { type: "i128" }),
      nativeToScVal(path, { type: "Vec" }),
      new Address(testAccount.publicKey()).toScVal(),
      nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }),
    ];
  
    await invokeContract(
      "router",
      addressBook,
      "swap_exact_tokens_for_tokens",
      swapParams,
      testAccount,
    );
  
    console.log(colors.cyan, "Fetching new token balances...")
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
    console.log('😩 > Error Swapping Tokens:', error);
  }
}