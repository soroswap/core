import { colors, tokenContracts, loadAccounts } from "./utils/utils";
import { TransactionBuilder } from "./utils/TransactionBuilder";
import { testAccount, tokenContract } from './utils/types'
const network = "standalone"
const rpcUri = "http://stellar"
const routerContractAddress = "CDPOUV6Q2DLFOQZ3TA5ACUADUWB56HCQAAJWLVV27INLWC7MVAJXY3EU"
const txBuilder = new TransactionBuilder(
    `${rpcUri}:8000`,
    `${rpcUri}:8000/soroban/rpc`,
    `${rpcUri}:8000/friendbot?addr=`,
    routerContractAddress,
    network
);
export const swap = async (txMaker: TransactionBuilder) => {
    console.log('')
    console.log(colors.purple, '===========')
    console.log(colors.purple, '= SWAP.ts =')
    console.log(colors.purple, '===========')
    console.log('')

    console.log(colors.cyan, "Fetching accounts...")
    const accounts = await loadAccounts() as testAccount[]
    const user = accounts[0] as testAccount
    console.log(colors.green, `Loaded user`)
    console.log(user)

    console.log(colors.cyan, "Funding account...")
    await txMaker.fundAccount(user)
    
    console.log(colors.cyan, "Fetching tokens...")
    const token_0 = tokenContracts[0] as tokenContract
    const token_1 = tokenContracts[1] as tokenContract
    console.log(colors.green, `Loaded tokens: ${token_0.symbol} and ${token_1.symbol}`)

    console.log(colors.cyan, "Fetching token balances...")
    const token_0_first_balance = await txMaker.getTokenBalance({source: user, contractId: token_0.contractId})
    const token_1_first_balance = await txMaker.getTokenBalance({source: user, contractId: token_1.contractId})

    console.log(colors.green, `${token_0.symbol}_Balance:`, token_0_first_balance)
    console.log(colors.green, `${token_1.symbol}_Balance:`, token_1_first_balance)

    console.log(colors.cyan, "Swapping tokens...")
    const swapTokensResponse = await txMaker.swapExactTokensForTokensSoroswap({
        source: user,
        amountIn: "100000",
        amountOutMin: "0",
        path: [token_0.contractId, token_1.contractId],
        to: user,
    }).catch((error) => { console.error(colors.red, "ERROR: couldn't swap tokens", error) })
    if(swapTokensResponse?.status == "SUCCESS") {
        console.log(colors.green, `Tokens swapped successfully`)
    }

    console.log(colors.cyan, "Fetching new token balances...")
    const token_0_last_balance = await txMaker.getTokenBalance({source: user, contractId: token_0.contractId})
    const token_1_last_balance = await txMaker.getTokenBalance({source: user, contractId: token_1.contractId})
    console.log(colors.green, `${token_0.symbol}_Balance:`, token_0_last_balance)
    console.log(colors.green, `${token_1.symbol}_Balance:`, token_1_last_balance)
    console.log('')
    console.log(colors.green, '- Done. -')
}
//swap(txBuilder)