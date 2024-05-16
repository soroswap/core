import { Address, nativeToScVal, xdr } from "@stellar/stellar-sdk";
import { getTokenBalance, invokeCustomContract } from "../../utils/contract.js";
import { colors } from "../../utils/index.js";
import { TokensBook } from "../../utils/tokens_book.js";

export async function mintTokens(network: string, tokensBook: TokensBook, loadedConfig: any) {
  try {
    console.log(colors.purple, '====================')
    console.log(colors.purple, '= Minting Tokens =')
    console.log(colors.purple, '====================')
    const testAccount = loadedConfig.getUser('TESTING_ACCOUNT_SECRET_KEY');
    const tokensAdmin = loadedConfig.getUser('TEST_TOKENS_ADMIN_SECRET_KEY');

    const tokens = tokensBook.getTokensByNetwork(network);
    const tokensToMint = [tokens![1], tokens![7]];

    for (let i = 0; i < tokensToMint.length; i++) {
      const token = tokensToMint[i];
      console.log(colors.green, `Minting ${token.code}`)
      const mintTokensParams: xdr.ScVal[] = [
        new Address(testAccount.publicKey()).toScVal(),
        nativeToScVal(50000000000, { type: 'i128' }),
      ]
  
      await invokeCustomContract(token.contract, 'mint', mintTokensParams, tokensAdmin);
      const tokenBalance = await getTokenBalance(
        token.contract,
        testAccount.publicKey(),
        testAccount,
      )

      console.log(
        colors.yellow,
        `${token.code} Balance: ${tokenBalance}`
      );
      
    }
  } catch (error) {
    console.log('😩 > Error minting Tokens:', error);
    
  }
}